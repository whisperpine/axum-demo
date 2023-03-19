use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::time::error::Elapsed;
use tower::BoxError;

pub async fn buffer_error_handler(_: BoxError) -> impl IntoResponse {
    (
        StatusCode::TOO_MANY_REQUESTS,
        "request count reaches the buffer limit",
    )
}

pub async fn timeout_error_handler(err: BoxError) -> impl IntoResponse {
    if err.is::<Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "request timed out".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal server error: {}", err),
        )
    }
}
