rust
assert_matches!(thing, Some((x, y)) => {
    assert_eq!(x.a, 10);
    assert_eq!(y.b(), 20);
});
