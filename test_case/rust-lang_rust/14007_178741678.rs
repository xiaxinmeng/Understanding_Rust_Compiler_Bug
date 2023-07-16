
use std::collections::BTreeMap;

fn main() {
    let mut a = BTreeMap::new();
    a.insert(0, 1);
    a.insert(0, "foo");
}
