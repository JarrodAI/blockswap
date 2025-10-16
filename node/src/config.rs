use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub p2p_port: u16,
    pub rpc_http_port: u16,
    pub rpc_ws_port: u16,
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            p2p_port: 30333,
            rpc_http_port: 9933,
            rpc_ws_port: 9944,
            db_path: "./.blockswap/data".into(),
        }
    }
}
