use crate::transaction::Transaction;
use crate::utils::sha256;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub block_hash: String,
    pub state_root: String,
    pub block_producer: String,
    pub metadata: Value,
}

#[derive(Debug)]
pub enum BlockError {
    InvalidMerkleRoot,
    InvalidBlockHash,
    InvalidStateRoot,
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockError::InvalidMerkleRoot => write!(f, "Invalid Merkle Root"),
            BlockError::InvalidBlockHash => write!(f, "Invalid Block Hash"),
            BlockError::InvalidStateRoot => write!(f, "Invalid State Root"),
        }
    }
}

impl Error for BlockError {}

impl Block {
    pub fn new(index: u32, previous_hash: String, transactions: Vec<Transaction>, block_producer: String, metadata: Value) -> Self {
        let timestamp = Utc::now().timestamp();
        let state_root = Self::calculate_state_root(&transactions);
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            block_hash: String::new(),
            state_root,
            block_producer,
            metadata,
        };
        block.block_hash = block.calculate_hash();
        block
    }

    // Merkle root calculation for the block's transactions
    fn calculate_state_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return sha256("");
        }
        let tx_hashes: Vec<String> = transactions.iter().map(|tx| tx.tx_hash.clone()).collect();
        Self::compute_merkle_root(tx_hashes)
    }

    fn compute_merkle_root(mut leaves: Vec<String>) -> String {
        if leaves.len() == 1 {
            return leaves[0].clone();
        }

        if leaves.len() % 2 != 0 {
            leaves.push(leaves.last().unwrap().clone());  // Duplicate the last hash if odd number of leaves
        }

        let mut new_level = Vec::new();
        for i in (0..leaves.len()).step_by(2) {
            let combined = format!("{}{}", leaves[i], leaves[i + 1]);
            new_level.push(sha256(&combined));
        }

        Self::compute_merkle_root(new_level)
    }

    pub fn calculate_hash(&self) -> String {
        let transactions_data: String = self.transactions.iter().map(|tx| tx.tx_hash.clone()).collect();
        let data = format!(
            "{}{}{}{}{}{}{}",
            self.index,
            self.previous_hash,
            self.timestamp,
            transactions_data,
            self.nonce,
            self.state_root,
            self.block_producer
        );
        sha256(&data)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.block_hash[..difficulty] != target {
            self.nonce += 1;
            self.block_hash = self.calculate_hash();
        }
    }

    pub fn validate_block(&self) -> Result<(), BlockError> {
        if !self.verify_merkle_root() {
            return Err(BlockError::InvalidMerkleRoot);
        }
        Ok(())
    }

    // Verify that the recomputed Merkle root matches the stored state_root
    fn verify_merkle_root(&self) -> bool {
        let recomputed_root = Self::calculate_state_root(&self.transactions);
        recomputed_root == self.state_root
    }
}
