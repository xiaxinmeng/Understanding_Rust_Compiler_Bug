 rust
let mut r = 0..10;
let array: [i32; 5] = (&mut r).collect();
assert_eq!(r.next(), Some(5));
