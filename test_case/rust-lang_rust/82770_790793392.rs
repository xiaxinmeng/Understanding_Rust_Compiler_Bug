rust
    let a = Some("asdf".to_string());
    assert_matches!(a, Some(_));
    dbg!(a); // OK, `a` still exists.
