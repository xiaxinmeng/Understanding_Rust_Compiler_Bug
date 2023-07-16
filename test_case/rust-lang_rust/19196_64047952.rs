 rust
let mut iter = other.move_iter();
vec.extend(iter.by_ref());
other = iter.unwrap();
