use anyhow::Result;
use tracing::info;

pub async fn start(cfg: &crate::config::Config) -> Result<()> {
    info!(http = cfg.rpc_http_port, ws = cfg.rpc_ws_port, "rpc stub started");
    Ok(())
}
