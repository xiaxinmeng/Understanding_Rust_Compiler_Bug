rust
use std::collections::BTreeMap;

fn main() {
    let x: [u16; 32768 * 4] = [0; 32768 * 4];
    let mut bm = BTreeMap::new();
    bm.insert(0, x);
}
