use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    nonce: u32,
    tx_hash: String,
}

impl Transaction {
    fn new(sender: String, receiver: String, amount: f64, nonce: u32) -> Self {
        let mut tx = Transaction {
            sender,
            receiver,
            amount,
            nonce,
            tx_hash: String::new(),
        };
        tx.tx_hash = tx.calculate_hash();
        tx
    }

    fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    index: u32,
    previous_hash: String,
    timestamp: i64,
    transactions: Vec<Transaction>,
    nonce: u64,
    block_hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, transactions: Vec<Transaction>) -> Self {
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

    fn calculate_hash(&self) -> String {
        let transactions_data: String = self.transactions.iter().map(|tx| tx.tx_hash.clone()).collect();
        let data = format!("{}{}{}{}{}", self.index, self.previous_hash, self.timestamp, transactions_data, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.block_hash[..difficulty] != target {
            self.nonce += 1;
            self.block_hash = self.calculate_hash();
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "0".to_string(), Vec::new());
        self.chain.push(genesis_block);
    }

    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn mine_pending_transactions(&mut self, miner_address: String) {
        let latest_block = self.get_latest_block();
        let new_block = Block::new(latest_block.index + 1, latest_block.block_hash.clone(), self.pending_transactions.clone());
        let mut mined_block = new_block;
        mined_block.mine_block(self.difficulty);
        self.chain.push(mined_block);
        self.pending_transactions = vec![Transaction::new("system".to_string(), miner_address, 1.0, 0)];
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.block_hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.block_hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
    blockchain.mine_pending_transactions("Miner1".to_string());

    blockchain.add_transaction(Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0, 1));
    blockchain.mine_pending_transactions("Miner2".to_string());

    println!("Blockchain: {:?}", blockchain.chain);
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
}
