#[cfg(test)]
mod tests {
    use jam::utils::sha256;

    #[test]
    fn test_sha256() {
        let data = "THIS IS MY JAM";
        let expected_hash = "2a8b50363a9292a36c144c38d8ee49370cc12fd27e630a88f0ffaaad481af054";
        assert_eq!(sha256(data), expected_hash);
    }
}
