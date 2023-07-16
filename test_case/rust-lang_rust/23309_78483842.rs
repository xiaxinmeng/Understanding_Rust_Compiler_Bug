 rust
let mut b = Box::new(5);
let old = std::mem::replace(&mut *b, 10);
