 rust
let mut s = String::new();
assert_eq!(s.len(), 0);
s.push_char('\u2022');
s.push_char('\U0001F419');
assert_eq!(s.len(), 2); // FAILS
