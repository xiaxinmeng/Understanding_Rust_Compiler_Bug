 rust
use std::hashmap::HashMap;
use std::to_bytes::{IterBytes, Cb};

impl IterBytes for char {
    pub fn iter_bytes(&self, lsb0: bool, f: Cb) -> bool {
        (self as i32).iter_bytes(lsb0, f)
    }
}

fn main() {
    let h = HashMap::new();
    h.insert('a', ());
    println(h.contains_key('a').to_str());
}
