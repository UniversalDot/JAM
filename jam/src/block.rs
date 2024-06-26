use crate::transaction::Transaction;
use crate::utils::sha256;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub block_hash: String,
}

impl Block {
    pub fn new(index: u32, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            block_hash: String::new(),
        };
        block.block_hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let transactions_data: String = self.transactions.iter().map(|tx| tx.tx_hash.clone()).collect();
        sha256(&format!("{}{}{}{}{}", self.index, self.previous_hash, self.timestamp, transactions_data, self.nonce))
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.block_hash[..difficulty] != target {
            self.nonce += 1;
            self.block_hash = self.calculate_hash();
        }
    }
}
