rust
use std::collections::BTreeMap;
use std::io::Read;

pub fn example() {
    let mut path_content_map: BTreeMap<&str, (u64, Box<dyn Read>)> = BTreeMap::new();
    let s = format!("");
    let as_bytes = s.as_bytes();
    path_content_map.insert("foo", (as_bytes.len() as u64, Box::new(as_bytes)));
}
