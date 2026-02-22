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

mod config;
/// From [`anyhow::Error`] to [`AppError`] which impl [`IntoResponse`].
mod error;
/// Handle [`axum::extract::Form`] request.
mod form;
/// MongoDB.
mod mongo;
/// Convert [`tower::Service`] inner error [`IntoResponse`].
mod service;
mod utils;

#[cfg(test)]
mod tests;

pub use config::{CRATE_NAME, PKG_VERSION, TIMEOUT_SECS};
pub use error::{Error, Result};
pub use form::{log_form, show_form};
pub use mongo::{list_collections, log_registered_users, register_user};
pub use service::{buffer_error_handler, timeout_error_handler};
pub use utils::{handler_404, handler_root, log_path, shutdown};
