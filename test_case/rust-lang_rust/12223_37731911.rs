 rust
let b: ~[&str] = a.lines().to_owned_vec();
drop(a);
