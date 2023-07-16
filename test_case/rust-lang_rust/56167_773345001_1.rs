rust
#![feature(hash_raw_entry)]

use std::collections::hash_map::{HashMap, RawEntryMut};

fn main() {
    let mut map = HashMap::new();
    if let RawEntryMut::Vacant(_) = map.raw_entry_mut().from_key(&42) {
        map.insert(1, 2);
    }
    println!("{}", map[&1]);
}
