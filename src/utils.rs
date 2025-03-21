use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use tracing::info;

/// "/" handler.
pub async fn handler_root() -> Html<&'static str> {
    Html(include_str!("../pages/index.html"))
}

/// Log url path.
pub async fn log_path(Path(value): Path<String>) {
    info!("{}", value);
}

/// Routing fallback.
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 not found")
}

/// Graceful shutdown
///
/// Shutdown the server when pressing `Ctrl+C`.
pub async fn shutdown() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("starting graceful shutdown");
}
