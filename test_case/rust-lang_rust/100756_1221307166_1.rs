rust
let mut iter = "a+++b".split_inclusive("++");
assert_eq!(iter.next(), Some("a++"));
assert_eq!(iter.next_back(), Some("+b")); // Fails
// This is actually equal to `Some("b")`
assert_eq!(iter.next(), None) // Fails
// This is actually equal to `Some("+")`
