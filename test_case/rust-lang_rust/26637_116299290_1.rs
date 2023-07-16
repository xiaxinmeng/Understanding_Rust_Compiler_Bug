 Rust
use std::env;
fn main() {
    let (vec1, mut vec0): (Vec<_>, Vec<&str>); // order doesn't matter this way
    vec1 = env::args().collect();
    vec0 = vec![];
    let s = vec1.get(0).unwrap();
    vec0.push(&s);
}
