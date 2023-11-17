#![forbid(unsafe_code)]
#![deny(clippy::disallowed_types)]

/// Set mimalloc as heap memory allocator when then `mimalloc` feature is enabled.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Program version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::net::SocketAddr;

    init_tracing_subscriber();
    print_libc_linkage();

    tracing::info!("app version: {}", VERSION);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening at http://localhost:{}", addr.port());

    axum::Server::bind(&addr)
        .serve(axum_demo::app().into_make_service())
        .with_graceful_shutdown(axum_demo::shutdown())
        .await?;

    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_demo=info".into()),
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
