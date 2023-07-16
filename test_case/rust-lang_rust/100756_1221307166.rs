rust
assert_eq!("a+++b".split_inclusive("++").collect::<Vec<_>>(), vec!["a++", "+b"]);
assert_eq!("a+++b".split_inclusive("++").rev().collect::<Vec<_>>(), vec!["+b", "a++"]); // Fails
// This is actually equal to `vec!["b", "a+++"]`
