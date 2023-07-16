rust
fn expect_iterator(_: impl Iterator) {}
fn test(into: impl IntoIterator) {
    expect_iterator(into)
}
