 rust
let mut m = HashMap::new();
m.insert(1i, 2i);
m.insert(3, 4);
assert_eq(m.find(&3).map(|&i| i).unwrap(), 4);
