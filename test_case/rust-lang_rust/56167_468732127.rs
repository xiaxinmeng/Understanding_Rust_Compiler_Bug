rust
#![feature(hash_raw_entry)]

use std::collections::HashMap;

let mut map = HashMap::new();

map.raw_entry_mut()
   .from_key("poneyland")
   .or_insert("poneyland", 3);
