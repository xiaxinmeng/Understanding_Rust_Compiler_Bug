rust

#[test]
fn pain() {
    let mut s = std::collections::BTreeMap::new();
    s.insert((), [0u8; 500_000]);
}
