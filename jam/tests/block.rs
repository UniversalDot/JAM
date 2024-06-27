#[cfg(test)]
mod tests {
    use super::*;
    use jam::transaction::Transaction;
    use jam::block::Block;
    use serde_json::json;
    use chrono::Utc;

    #[test]
    fn test_new_block() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let metadata = json!({
            "miner": "Miner1",
            "timestamp": Utc::now().to_rfc3339(),
        });
        let block = Block::new(1, "0".to_string(), transactions.clone(), "Miner1".to_string(), metadata.clone());
        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "0".to_string());
        // assert_eq!(block.transactions, transactions);
        assert_eq!(block.block_producer, "Miner1");
        assert_eq!(block.metadata, metadata);
    }

    #[test]
    fn test_calculate_hash() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let metadata = json!({
            "miner": "Miner1",
            "timestamp": Utc::now().to_rfc3339(),
        });
        let block = Block::new(1, "0".to_string(), transactions.clone(), "Miner1".to_string(), metadata);
        let expected_hash = block.calculate_hash();
        assert_eq!(block.block_hash, expected_hash);
    }

    #[test]
    fn test_mine_block() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let metadata = json!({
            "miner": "Miner1",
            "timestamp": Utc::now().to_rfc3339(),
        });
        let mut block = Block::new(1, "0".to_string(), transactions.clone(), "Miner1".to_string(), metadata);
        block.mine_block(2);
        assert!(block.block_hash.starts_with("00"));
    }
}
