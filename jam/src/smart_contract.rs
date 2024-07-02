use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::utils::sha256;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct SmartContract {
    pub code: String,
    pub state: serde_json::Value,
    pub address: String,
}

impl SmartContract {
    pub fn new(code: String, state: serde_json::Value) -> Self {
        let address = sha256(&format!("{}{}", code, state));
        SmartContract { code, state, address }
    }

    pub fn execute(&mut self, transaction: &Transaction) {
        if let Some(state_map) = self.state.as_object_mut() {
            if let Some(sender_balance) = state_map.get_mut(&transaction.sender) {
                *sender_balance = json!(sender_balance.as_f64().unwrap_or(0.0) - transaction.amount);
            } else {
                state_map.insert(transaction.sender.clone(), json!(-transaction.amount));
            }

            if let Some(receiver_balance) = state_map.get_mut(&transaction.receiver) {
                *receiver_balance = json!(receiver_balance.as_f64().unwrap_or(0.0) + transaction.amount);
            } else {
                state_map.insert(transaction.receiver.clone(), json!(transaction.amount));
            }
        }
    }
}
