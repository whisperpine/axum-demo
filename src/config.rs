use std::sync::LazyLock;

/// Program version.
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

/// Environment variable named `TIMEOUT_SECS`
const ENV_TIMEOUT_SECS: &str = "TIMEOUT_SECS";

/// Timeout seconds for server internal process.
pub static TIMEOUT_SECS: LazyLock<f32> = LazyLock::new(get_timeout_secs);

fn get_timeout_secs() -> f32 {
    match std::env::var(ENV_TIMEOUT_SECS) {
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
    }
}
