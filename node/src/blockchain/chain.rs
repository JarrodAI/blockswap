//! Chain logic (minimal)

use super::{block::Block, state::State};
use anyhow::{bail, Result};

#[derive(Debug, Default)]
pub struct Chain {
    pub state: State,
    pub height: u64,
}

impl Chain {
    pub fn new() -> Self { Self::default() }

    pub fn apply_block(&mut self, block: Block) -> Result<()> {
        let computed_root = super::block::Block::compute_tx_root(&block.txs);
        if block.header.tx_root != computed_root {
            bail!("tx_root mismatch");
        }

        for tx in &block.txs {
            self.state.apply_transaction(tx).map_err(|e| anyhow::anyhow!(e))?;
        }

        self.height = block.header.height;
        Ok(())
    }

    pub fn get_height(&self) -> u64 { self.height }
}
