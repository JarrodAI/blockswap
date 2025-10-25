#[test]
fn node_scaffold_runs() {
    // Placeholder test to ensure test harness is set up.
    assert!(true);
}

use blockswap_node::blockchain::{block::{Block, BlockHeader}, chain::Chain, transaction::{Transaction, TxType}};

#[test]
fn apply_block_with_transfer_updates_state() {
    let mut chain = Chain::new();

    // genesis: fund sender
    let sender: [u8; 32] = [1; 32];
    let receiver: [u8; 32] = [2; 32];
    {
        let acc = chain.state.ensure_account(&sender);
        acc.balance = 1_000_000;
    }

    let tx = Transaction {
        from: sender,
        to: receiver,
        amount: 1000,
        fee: 10,
        nonce: 0,
        tx_type: TxType::Transfer,
        payload: vec![],
    };

    let txs = vec![tx.clone()];
    let header = BlockHeader {
        parent_hash: [0u8; 32],
        state_root: [0u8; 32], // unused in this test
        tx_root: Block::compute_tx_root(&txs),
        height: 1,
        timestamp: 0,
    };
    let block = Block { header, txs };

    chain.apply_block(block).expect("apply block");

    // sender decreased by amount+fee, nonce incremented
    let s = chain.state.get_account(&sender).unwrap();
    assert_eq!(s.balance, 1_000_000 - 1010);
    assert_eq!(s.nonce, 1);

    // receiver increased by amount
    let r = chain.state.get_account(&receiver).unwrap();
    assert_eq!(r.balance, 1000);
}
