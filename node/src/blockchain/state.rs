//! Minimal state management and STF

use crate::blockchain::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::transaction::{Transaction, TxType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Account {
    pub balance: u128,
    pub nonce: u64,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub accounts: HashMap<Address, Account>,
}

#[derive(thiserror::Error, Debug)]
pub enum StateError {
    #[error("account not found")]
    AccountNotFound,
    #[error("insufficient balance")]
    InsufficientBalance,
    #[error("invalid nonce: expected {expected}, got {got}")]
    InvalidNonce { expected: u64, got: u64 },
}

impl State {
    pub fn get_account(&self, addr: &Address) -> Option<&Account> {
        self.accounts.get(addr)
    }

    pub fn ensure_account(&mut self, addr: &Address) -> &mut Account {
        self.accounts.entry(*addr).or_default()
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), StateError> {
        match tx.tx_type {
            TxType::Transfer => {
                // Sender must exist
                let sender = self
                    .accounts
                    .get_mut(&tx.from)
                    .ok_or(StateError::AccountNotFound)?;

                if sender.nonce != tx.nonce {
                    return Err(StateError::InvalidNonce { expected: sender.nonce, got: tx.nonce });
                }

                let total = tx
                    .amount
                    .checked_add(tx.fee)
                    .ok_or(StateError::InsufficientBalance)?;

                if sender.balance < total {
                    return Err(StateError::InsufficientBalance);
                }

                sender.balance = sender.balance - total;
                sender.nonce = sender.nonce.saturating_add(1);

                // borrow checker friendly: after finished using sender, we can borrow receiver
                let recv_acc = self.ensure_account(&tx.to);
                recv_acc.balance = recv_acc.balance.saturating_add(tx.amount);
                Ok(())
            }
        }
    }
}
