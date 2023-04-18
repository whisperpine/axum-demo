use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
use axum::Router;
use axum_demo::mongo;
use axum_demo::service::{buffer_error_handler, timeout_error_handler};
use axum_demo::*;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// set mimalloc as heap memory allocator
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // info!("listening at http://{}", addr);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening at http://localhost:{}", addr.port());

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .with_graceful_shutdown(shutdown())
        .await?;

    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_demo=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn app() -> Router {
    Router::new()
        .route("/", get(handler_root).post(register_user))
        .route("/mongo", get(mongo::log_mongo))
        .route("/form", get(show_form).post(log_form))
        .route("/path/:path_id", get(log_path))
        .route("/error", get(app_error))
        .fallback(handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(buffer_error_handler))
                .buffer(100)
                .layer(HandleErrorLayer::new(timeout_error_handler))
                .timeout(Duration::from_millis(500)),
        )
}
