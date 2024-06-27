#[cfg(test)]
mod tests {
    use super::*;
    use jam::transaction::Transaction;
    use jam::blockchain::Blockchain;
    // use serde_json::json;

    #[test]
    fn test_create_genesis_block() {
        let blockchain = Blockchain::new(2);
        assert_eq!(blockchain.chain.len(), 1);
        let genesis_block = &blockchain.chain[0];
        assert_eq!(genesis_block.index, 0);
        assert_eq!(genesis_block.transactions.len(), 1);
        assert_eq!(genesis_block.transactions[0].sender, "system");
        assert_eq!(genesis_block.transactions[0].receiver, "genesis");
        assert_eq!(genesis_block.transactions[0].amount, 0.0);
        assert_eq!(genesis_block.block_producer, "genesis_producer");
        assert_eq!(genesis_block.metadata["description"], "Genesis block");
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new(2);
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1);
        blockchain.add_transaction(transaction.clone());
        assert_eq!(blockchain.pending_transactions.len(), 1);
        // assert_eq!(blockchain.pending_transactions[0], transaction);
    }

    #[test]
    fn test_mine_pending_transactions() {
        let mut blockchain = Blockchain::new(2);
        blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
        blockchain.mine_pending_transactions("Miner1".to_string());
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.pending_transactions.len(), 1);
        assert_eq!(blockchain.pending_transactions[0].receiver, "Miner1");
        let new_block = &blockchain.chain[1];
        assert_eq!(new_block.block_producer, "Miner1");
        assert!(new_block.metadata["timestamp"].is_string());
    }

    #[test]
    fn test_is_chain_valid() {
        let mut blockchain = Blockchain::new(2);
        blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
        blockchain.mine_pending_transactions("Miner1".to_string());
        assert!(blockchain.is_chain_valid());
    }
}
