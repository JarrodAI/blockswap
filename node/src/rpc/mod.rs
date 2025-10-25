use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub mod server;
pub mod handlers;

pub async fn start(cfg: &crate::config::Config) -> Result<()> {
    info!(http = cfg.rpc_http_port, ws = cfg.rpc_ws_port, "starting rpc");
    let shared_chain = Arc::new(Mutex::new(crate::blockchain::chain::Chain::new()));
    server::start_http_server(cfg.rpc_http_port, shared_chain).await?;
    Ok(())
}
