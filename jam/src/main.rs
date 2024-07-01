mod block;
mod blockchain;
mod smart_contract;
mod transaction;
mod utils;
mod config;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::smart_contract::SmartContract;
use crate::config::Config;
use serde_json::json;
use tokio::time::{self, Duration};

fn main() {
    let config = Config::new(6, 2, 50.0, "genesis_producer".to_string()); // Set configurations

    let mut blockchain = Blockchain::new(config.difficulty, config.block_reward);

    // Add initial transactions
    blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
    blockchain.mine_pending_transactions("Miner1".to_string());

    blockchain.add_transaction(Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0, 1));
    blockchain.mine_pending_transactions("Miner2".to_string());

    println!("Initial Blockchain: {:?}\n", blockchain.chain);
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());

    // Deploy a smart contract
    let contract_code = String::from("transfer");
    let contract_state = json!({});

    let mut contract = SmartContract::new(contract_code, contract_state);
    println!("Deployed Contract: {:?}", contract);

    blockchain.add_transaction(Transaction::new("Alice".to_string(), contract.address.clone(), 10.0, 1));
    blockchain.mine_pending_transactions("Miner1".to_string());

    contract.execute(&Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
    println!("Contract State after Execution: {:?}", contract.state);

    // Create a runtime and spawn the block production task
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let blockchain = std::sync::Arc::new(tokio::sync::Mutex::new(blockchain));
    let interval = config.block_production_interval;
    runtime.spawn(start_block_production(blockchain.clone(), interval));

    // Keep the main function alive
    runtime.block_on(async {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

async fn start_block_production(blockchain: std::sync::Arc<tokio::sync::Mutex<Blockchain>>, interval: u64) {
    let mut interval = time::interval(Duration::from_secs(interval));
    loop {
        interval.tick().await;
        let mut blockchain = blockchain.lock().await;
        blockchain.mine_pending_transactions("AutoMiner".to_string());
        println!("New Block Mined: {:?}\n", blockchain.get_latest_block());
    }
}
