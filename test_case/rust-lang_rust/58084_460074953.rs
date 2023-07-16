rust
let a = [1, 2, 3, 4, 5];
let b = &a[1..4];
assert_eq!(b.len(), 3);
// No way to derive a.len() from b!
