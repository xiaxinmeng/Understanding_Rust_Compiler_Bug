rust
    let a = Some("asdf".to_string());
    assert_matches!(a, Some(_x));
    dbg!(a); // Error, `a` was consumed by the pattern.
