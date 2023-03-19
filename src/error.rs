use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Wraps [`anyhow::Error`] in order to response
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("anyhow error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        AppError(value.into())
    }
}
