#[cfg(test)]
mod tests {
    use jam::block::Block;
    use jam::transaction::Transaction;

    #[test]
    fn test_new_block() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let block = Block::new(1, "0".to_string(), transactions.clone());
        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "0".to_string());
    }

    #[test]
    fn test_calculate_hash() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let block = Block::new(1, "0".to_string(), transactions.clone());
        let expected_hash = block.calculate_hash();
        assert_eq!(block.block_hash, expected_hash);
    }

    #[test]
    fn test_mine_block() {
        let transactions = vec![Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1)];
        let mut block = Block::new(1, "0".to_string(), transactions.clone());
        block.mine_block(2);
        assert!(block.block_hash.starts_with("00"));
    }
}
