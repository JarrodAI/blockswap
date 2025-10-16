mod config;
mod blockchain;
mod consensus;
mod crosschain;
mod swap;
mod crypto;
mod network;
mod storage;
mod mempool;
mod vm;
mod rpc;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    info!("starting BlockSwap node (scaffold)");

    // Load config (defaults for now)
    let cfg = config::Config::default();
    info!(?cfg, "loaded config");

    // Start subsystems (stubbed)
    storage::init(&cfg)?;
    network::start(&cfg).await?;
    rpc::start(&cfg).await?;

    // Wait for shutdown signal (Ctrl+C)
    tokio::signal::ctrl_c().await?;

    info!("shutting down");
    Ok(())
}
