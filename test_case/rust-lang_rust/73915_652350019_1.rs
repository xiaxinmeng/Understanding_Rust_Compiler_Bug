rust
fn main() {
    let mut a = std::collections::BTreeMap::new();
    a.insert(1, 1);
    a.insert(2, 2);
    a.iter_mut().map(|(_,v)| v).min();
}
