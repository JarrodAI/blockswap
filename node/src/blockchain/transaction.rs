//! Transaction types and structure (minimal)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxType {
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub amount: u128,
    pub fee: u128,
    pub nonce: u64,
    pub tx_type: TxType,
    #[serde(default)]
    pub payload: Vec<u8>,
}

impl Transaction {
    pub fn hash(&self) -> [u8; 32] {
        // Simple canonical hash: blake3 over JSON bytes (sufficient for prototype)
        let json = serde_json::to_vec(self).expect("tx serde");
        blake3::hash(&json).into()
    }
}
