 rust
nocapture = match env::var("RUST_TEST_NOCAPTURE") {
            Ok(val) => &val != "0",                                             
            Err(_) => false
        };
