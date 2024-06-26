use crate::utils::sha256;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub nonce: u32,
    pub tx_hash: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64, nonce: u32) -> Self {
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

    pub fn calculate_hash(&self) -> String {
        sha256(&format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.nonce))
    }
}
