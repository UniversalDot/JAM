#[cfg(test)]
mod tests {
    
    use jam::transaction::Transaction;
    use jam::block::Block;
    use jam::blockchain::Blockchain;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_automatic_block_creation() {
        let blockchain = Arc::new(Mutex::new(Blockchain::new(2, 50.0)));

        // Start block production
        let blockchain_clone = blockchain.clone();
        tokio::spawn(async move {
            start_block_production(blockchain_clone).await;
        });

        // Wait for a short period to allow blocks to be created
        tokio::time::sleep(Duration::from_secs(12)).await;

        // Check if new blocks have been created
        let blockchain = blockchain.lock().await;
        assert!(blockchain.chain.len() > 1, "No new blocks were created");
    }

    async fn start_block_production(blockchain: Arc<Mutex<Blockchain>>) {
        let mut interval = time::interval(Duration::from_secs(6));
        loop {
            interval.tick().await;
            let mut blockchain = blockchain.lock().await;
            blockchain.mine_pending_transactions("AutoMiner".to_string());
            println!("New Block Mined: {:?}\n", blockchain.get_latest_block());
        }
    }

    #[test]
    fn test_new_block() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let metadata = serde_json::json!({
            "miner": "Miner1",
            "timestamp": chrono::Utc::now().to_rfc3339(),
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
        let metadata = serde_json::json!({
            "miner": "Miner1",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        let block = Block::new(1, "0".to_string(), transactions.clone(), "Miner1".to_string(), metadata);
        let expected_hash = block.calculate_hash();
        assert_eq!(block.block_hash, expected_hash);
    }

    #[test]
    fn test_mine_block() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let metadata = serde_json::json!({
            "miner": "Miner1",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        let mut block = Block::new(1, "0".to_string(), transactions.clone(), "Miner1".to_string(), metadata);
        block.mine_block(2);
        assert!(block.block_hash.starts_with("00"));
    }
}
