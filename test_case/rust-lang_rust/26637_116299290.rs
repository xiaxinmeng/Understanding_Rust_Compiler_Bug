 Rust
use std::env;
fn main() {
    let vec1: Vec<_> = env::args().collect();
    let mut vec0: Vec<&str> = vec![]; // NOTE: ordering
    let s = vec1.get(0).unwrap();
    vec0.push(&s);
}
