 rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let key = 7u;
    let item = map.get(&key);
    if (item.is_some() && item.unwrap().eq(8i)) {
        return;
    }
    map.insert(key, 7u);
}
