rust
    assert_eq!(Arc::strong_count(&one_ref), 2);
    assert_eq!(Arc::weak_count(&one_ref), 1);
