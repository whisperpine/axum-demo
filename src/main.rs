//! Demo project for [axum](https://github.com/tokio-rs/axum) based web server.

#![cfg_attr(
    not(debug_assertions),
    deny(warnings, missing_docs),
    deny(clippy::todo, clippy::unwrap_used)
)]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::net::SocketAddr;

    init_tracing_subscriber();
    print_libc_linkage();

    tracing::info!("App version: {}.", axum_demo::PKG_VERSION);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening at http://localhost:{}.", addr.port());

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app()).await?;

    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", axum_demo::CRATE_NAME).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(target_feature = "crt-static")]
fn print_libc_linkage() {
    tracing::info!("The C runtime is linked statically.");
}
#[cfg(not(target_feature = "crt-static"))]
fn print_libc_linkage() {
    tracing::info!("The C runtime is linked dynamically.");
}

fn app() -> axum::Router {
    use axum::error_handling::HandleErrorLayer;
    use axum::routing::get;
    use axum_demo::{
        TIMEOUT_SECS, buffer_error_handler, handler_404, handler_root, log_form, log_path,
        log_registered_users, register_user, show_form, timeout_error_handler,
    };
    use std::time::Duration;
    use tower::ServiceBuilder;

    axum::Router::new()
        .route("/", get(handler_root))
        .route("/mongo", get(log_registered_users).post(register_user))
        .route("/form", get(show_form).post(log_form))
        .route("/path/{path_id}", get(log_path))
        .fallback(handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(buffer_error_handler))
                .buffer(100)
                .layer(HandleErrorLayer::new(timeout_error_handler))
                .timeout(Duration::from_secs_f32(*TIMEOUT_SECS)),
        )
}
