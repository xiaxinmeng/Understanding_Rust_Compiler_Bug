
#[test]
fn test_iter_mut_map_min() {
    let mut a = BTreeMap::new();
    a.insert(1, 1);
    a.insert(2, 2);
    a.iter_mut().map(|(_,v)| v).min();
}
