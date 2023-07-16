rust
#![feature(map_entry_replace, entry_insert)]
use std::collections::HashMap;

fn main() {
    HashMap::new().entry(0).insert(0).replace_key();
}
