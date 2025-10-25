use anyhow::Result;
use tracing::info;

pub mod p2p;
pub mod gossip;
pub mod sync;

pub async fn start(cfg: &crate::config::Config) -> Result<()> {
    info!(p2p_port = cfg.p2p_port, "network stub started");
    Ok(())
}
