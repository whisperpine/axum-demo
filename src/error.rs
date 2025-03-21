use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// A handy type alias for `Result<T, axum_demo::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Enumeration of errors that can occur in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error raised by mongodb crate.
    #[error("Mongodb: {0}")]
    Mongodb(#[from] mongodb::error::Error),
    /// Execution timeout.
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {self}")).into_response()
    }
}
