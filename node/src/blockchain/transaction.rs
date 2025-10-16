//! Transaction types (stub)

#[derive(Debug, Clone)]
pub enum TxType {
    Transfer,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub amount: u64,
    pub nonce: u64,
    pub tx_type: TxType,
}
