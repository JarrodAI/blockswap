//! Block structure (minimal)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub tx_root: [u8; 32],
    pub height: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<crate::blockchain::transaction::Transaction>,
}

impl Block {
    pub fn compute_tx_root(txs: &[crate::blockchain::transaction::Transaction]) -> [u8; 32] {
        // Minimal root: blake3 hash of concatenated tx hashes
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        for tx in txs {
            let h = tx.hash();
            hasher.update(&h);
        }
        hasher.finalize().into()
    }
}
