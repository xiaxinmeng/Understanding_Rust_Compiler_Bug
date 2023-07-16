rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    map.insert("foo", "bar");
    let x = &format!("foo");
    let y = map[x];
}
