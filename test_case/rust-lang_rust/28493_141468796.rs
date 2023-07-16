 rust
#![feature(alloc_system)]
extern crate alloc_system;

use std::collections::BTreeMap;

fn main() {
    let _: BTreeMap<(), ()> = BTreeMap::new();
}
