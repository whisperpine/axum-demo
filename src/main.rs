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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::net::SocketAddr;

    init_tracing_subscriber();
    print_libc_linkage();

    tracing::info!("app version: {}", axum_demo::PKG_VERSION);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening at http://localhost:{}", addr.port());

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, axum_demo::app()).await?;

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
    tracing::info!("the C runtime is linked statically");
}
#[cfg(not(target_feature = "crt-static"))]
fn print_libc_linkage() {
    tracing::info!("the C runtime is linked dynamically");
}
