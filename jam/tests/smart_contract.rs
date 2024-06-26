#[cfg(test)]
mod tests {
    use jam::smart_contract::SmartContract;
    use jam::transaction::Transaction;
    use serde_json::json;
    use jam::utils::sha256;

    #[test]
    fn test_new_smart_contract() {
        let code = String::from("transfer");
        let state = json!({});
        let contract = SmartContract::new(code.clone(), state.clone());
        let expected_address = sha256(&format!("{}{}", code, state.to_string()));
        assert_eq!(contract.code, code);
        assert_eq!(contract.state, state);
        assert_eq!(contract.address, expected_address);
    }

    #[test]
    fn test_execute_contract() {
        let code = String::from("transfer");
        let state = json!({
            "Alice": 100.0,
            "Bob": 50.0
        });
        let mut contract = SmartContract::new(code.clone(), state);
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0, 1);
        contract.execute(&transaction);
        assert_eq!(contract.state["Alice"], json!(90.0));
        assert_eq!(contract.state["Bob"], json!(60.0));
    }
}
