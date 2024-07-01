use crate::block::Block;
use crate::transaction::Transaction;
use serde_json::json;
use chrono::Utc;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_transaction = Transaction::new("system".to_string(), "genesis".to_string(), 0.0, 0);
        let metadata = json!({
            "description": "Genesis block",
            "timestamp": Utc::now().to_rfc3339(),
        });
        let genesis_block = Block::new(0, "0".to_string(), vec![genesis_transaction], "genesis_producer".to_string(), metadata);
        self.chain.push(genesis_block);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let latest_block = self.get_latest_block();
        let metadata = json!({
            "miner": miner_address.clone(),
            "timestamp": Utc::now().to_rfc3339(),
        });
        let mut new_block = Block::new(
            latest_block.index + 1,
            latest_block.block_hash.clone(),
            self.pending_transactions.clone(),
            miner_address.clone(),
            metadata
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
        self.pending_transactions = vec![Transaction::new("system".to_string(), miner_address, 1.0, 0)];
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.block_hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.block_hash {
                return false;
            }
            if current_block.state_root != calculate_state_root(current_block.transactions.clone()) {
                return false;
            }
        }
        true
    }
}
