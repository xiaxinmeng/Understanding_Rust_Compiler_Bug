 rust
use std::collections::HashMap;
use twox_hash::RandomXxHashState;

let mut hash: HashMap<_, _, RandomXxHashState> = Default::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
