rust
let key = map.iter().nth(rand() % map.len()).0.clone();
map.remove(&key);
