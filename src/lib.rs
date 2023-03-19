/// From [`anyhow::Error`] to [`AppError`] which impl [`IntoResponse`]
pub mod error;
/// Handle [`axum::extract::Form`] request
pub mod form;
/// Convert [`tower::Service`] inner error [`IntoResponse`]
pub mod service;

use error::AppError;
pub use form::{log_form, show_form};

use anyhow::Result;
use axum::extract::{Form, Json, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

/// Graceful shutdown
///
/// Shutdown the server when pressing Ctrl+C.
pub async fn shutdown() {
    tokio::signal::ctrl_c().await.unwrap();
    info!("shutdown");
}

/// Routing fallback
pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 not found").into_response()
}

/// "/" handler
pub async fn handler_root() -> Html<&'static str> {
    // use std::time::Duration;
    // tokio::time::sleep(Duration::from_millis(300)).await;
    Html(include_str!("../index.html"))
}

pub async fn log_path(Path(value): Path<String>) {
    info!("{}", value);
}

/// Intend to response an error
pub async fn app_error() -> Result<(), AppError> {
    fn inner_error() -> Result<()> {
        anyhow::bail!("rua")
    }
    inner_error()?;
    Ok(())
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct UserInfo {
    username: String,
    id: Uuid,
}

pub async fn register_user(Form(value): Form<CreateUser>) -> Json<UserInfo> {
    let user_info = UserInfo {
        username: value.username,
        id: Uuid::new_v4(),
    };
    Json(user_info)
}
