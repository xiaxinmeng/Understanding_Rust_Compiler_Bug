 rust
use std::collections::HashMap;

fn main() {
    let s = [(1i, 1u), (2, 2), (3, 3)];
    let v: Vec<_> = s.iter().collect();
    let m: HashMap<_, _> = s.iter().map(|&x| x).collect();
}
