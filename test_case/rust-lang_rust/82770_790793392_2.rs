rust
    let a = Some("asdf".to_string());
    assert_matches!(a, Some(ref _x));
    dbg!(a); // OK, `a` still exists.
