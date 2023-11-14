#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::disallowed_types)]

/// Set mimalloc as heap memory allocator when then `mimalloc` feature is enabled.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Program version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::net::SocketAddr;

    init_tracing_subscriber();
    print_libc_linkage();

    tracing::info!("app version: {}", VERSION);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening at http://localhost:{}", addr.port());

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .with_graceful_shutdown(axum_demo::shutdown())
        .await?;

    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_demo=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(target_feature = "crt-static")]
fn print_libc_linkage() {
    tracing::info!("the C runtime is linked statically");
}
#[cfg(not(target_feature = "crt-static"))]
fn print_libc_linkage() {
    tracing::info!("the C runtime is linked dynamically");
}

fn app() -> axum::Router {
    use axum::error_handling::HandleErrorLayer;
    use axum::routing::get;
    use axum_demo::service::{buffer_error_handler, timeout_error_handler};
    use axum_demo::*;
    use std::time::Duration;
    use tower::ServiceBuilder;

    axum::Router::new()
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
