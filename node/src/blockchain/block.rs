//! Block structure (stub)

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub parent_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub tx_root: [u8; 32],
    pub height: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<crate::blockchain::transaction::Transaction>,
}
