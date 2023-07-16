rust
#![feature(hash_raw_entry)]

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.raw_entry_mut().from_key(&42).or_insert(1, 2);
    println!("{}", map[&1]);
}
