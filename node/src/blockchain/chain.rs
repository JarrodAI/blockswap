//! Chain logic (stub)

use super::{block::Block, state::State};

pub struct Chain {
    pub state: State,
    pub height: u64,
}

impl Chain {
    pub fn new() -> Self {
        Self { state: State::default(), height: 0 }
    }

    pub fn apply_block(&mut self, _block: Block) {
        self.height += 1;
    }
}
