#![forbid(unsafe_code)]
#![deny(clippy::disallowed_types)]

/// Set mimalloc as heap memory allocator when then `mimalloc` feature is enabled.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
