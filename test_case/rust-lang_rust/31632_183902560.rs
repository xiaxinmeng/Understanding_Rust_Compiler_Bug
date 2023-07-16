 Rust
    if let Ok(mut x) = "3.1415".parse::<f64>() {
        assert_eq!(8.1415, { x += 5.0; x });
    }
