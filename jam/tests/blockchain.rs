#[cfg(test)]
mod tests {
    use jam::blockchain::Blockchain;
    use jam::transaction::Transaction;

    #[test]
    fn test_create_genesis_block() {
        let mut blockchain = Blockchain::new(2);
        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].transactions.len(), 1);
        assert_eq!(blockchain.chain[0].transactions[0].sender, "system");
        assert_eq!(blockchain.chain[0].transactions[0].receiver, "genesis");
        assert_eq!(blockchain.chain[0].transactions[0].amount, 0.0);
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new(2);
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1);
        blockchain.add_transaction(transaction.clone());
        assert_eq!(blockchain.pending_transactions.len(), 1);
        // TODO fix
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
    }

    #[test]
    fn test_is_chain_valid() {
        let mut blockchain = Blockchain::new(2);
        blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1));
        blockchain.mine_pending_transactions("Miner1".to_string());
        assert!(blockchain.is_chain_valid());
    }
}
