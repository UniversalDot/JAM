mod blockchain;
mod block;
mod transaction;
mod smart_contract;
mod utils;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::smart_contract::SmartContract;
use serde_json::json;

fn main() {
    let mut blockchain = Blockchain::new(2);

    blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
    blockchain.mine_pending_transactions("Miner1".to_string());

    blockchain.add_transaction(Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0, 1));
    blockchain.mine_pending_transactions("Miner2".to_string());

    println!("Blockchain: {:?}", blockchain.chain);
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());

    let contract_code = String::from("transfer");
    let contract_state = json!({});

    let mut contract = SmartContract::new(contract_code, contract_state);
    println!("Deployed Contract: {:?}", contract);

    blockchain.add_transaction(Transaction::new("Alice".to_string(), contract.address.clone(), 10.0, 1));
    blockchain.mine_pending_transactions("Miner1".to_string());

    contract.execute(&Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
    println!("Contract State after Execution: {:?}", contract.state);
}
