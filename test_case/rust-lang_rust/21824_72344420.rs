 rust
#[test]
#[should_panic(expected = "out of bounds")]
fn test_oob() {
    vec![][10]
}
