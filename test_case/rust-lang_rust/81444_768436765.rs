
use std::collections::BTreeMap;

fn main() {
    let mut s = BTreeMap::new();
    s.insert((), [0u8; 37600]);
}
