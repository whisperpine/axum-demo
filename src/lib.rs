//! Demo project for [axum](https://github.com/tokio-rs/axum) based web server.

// rustc
#![cfg_attr(debug_assertions, allow(unused))]
#![cfg_attr(not(debug_assertions), warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(clippy::unwrap_used))]
#![cfg_attr(not(debug_assertions), deny(warnings))]
// clippy
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

/// From [`anyhow::Error`] to [`AppError`] which impl [`IntoResponse`]
pub mod error;
/// Handle [`axum::extract::Form`] request
pub mod form;
/// MongoDB
pub mod mongo;
/// Convert [`tower::Service`] inner error [`IntoResponse`]
pub mod service;

#[cfg(test)]
mod tests;

use error::AppError;
pub use form::{log_form, show_form};

use anyhow::Result;
use axum::extract::{Form, Json, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::info;
use uuid::Uuid;

/// Program version.
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

/// Environment variable named `TIMEOUT_SECS`
const ENV_TIMEOUT_SECS: &str = "TIMEOUT_SECS";

// static NICE: LazyLock<f32> = LazyLock::new()

/// Timeout seconds for server internal process.
static TIMEOUT_SECS: LazyLock<f32> = LazyLock::new(|| match std::env::var(ENV_TIMEOUT_SECS) {
    Ok(value) => match value.parse() {
        Ok(value) => {
            tracing::info!("server internal process timeout in seconds: {}", value);
            value
        }
        Err(err) => {
            tracing::error!("failed to parse ENV_TIMEOUT_SECS to f32");
            panic!("{}", err);
        }
    },
    Err(_) => {
        let value = 0.5;
        tracing::info!(
            "{} hasn't been set (default value: {})",
            ENV_TIMEOUT_SECS,
            value
        );
        value
    }
});

pub fn app() -> axum::Router {
    use crate::service::{buffer_error_handler, timeout_error_handler};
    use axum::error_handling::HandleErrorLayer;
    use axum::routing::get;
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
                .timeout(Duration::from_secs_f32(*TIMEOUT_SECS)),
        )
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

/// Routing fallback
pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 not found").into_response()
}

/// "/" handler
pub async fn handler_root() -> Html<&'static str> {
    Html(include_str!("../pages/index.html"))
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

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    id: Uuid,
}

/// Add [`UserInfo`] to database and response in json format.
pub async fn register_user(Form(value): Form<CreateUser>) -> Result<Json<UserInfo>, AppError> {
    let user_info = UserInfo {
        username: value.username,
        id: Uuid::new_v4(),
    };
    mongo::insert_userinfo(&user_info).await?;
    Ok(Json(user_info))
}

#[derive(Debug)]
pub enum Version {
    V1,
    V2,
    V3,
}

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;

#[axum::async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        use ahash::AHashMap;

        let params: Path<AHashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}
