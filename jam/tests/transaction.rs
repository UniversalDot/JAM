#[cfg(test)]
mod tests {
    use jam::transaction::Transaction;
    use jam::utils::sha256;

    #[test]
    fn test_new_transaction() {
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 100.0, 1);
        assert_eq!(transaction.sender, "Alice");
        assert_eq!(transaction.receiver, "Bob");
        assert_eq!(transaction.amount, 100.0);
        assert_eq!(transaction.nonce, 1);
    }

    #[test]
    fn test_calculate_hash() {
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1);
        let expected_hash = sha256(&format!("{}{}{}{}", "Alice", "Bob", 10.0, 1));
        assert_eq!(transaction.tx_hash, expected_hash);
    }
}
