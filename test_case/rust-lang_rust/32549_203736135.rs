 rust
nocapture = env::var("RUST_TEST_NOCAPTURE").unwrap_or("0".to_string()) != "0";
