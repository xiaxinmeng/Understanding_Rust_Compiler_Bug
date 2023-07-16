rust
fn foo<'a, K, V, I: 'a + IntoIterator<Item = (&'a K, &'a V)>>(i: I) {}

fn bar() {
    let map = HashMap::<u32, u32>::new();
    foo(&map);
    
    let map = BTreeMap::<u32, u32>::new();
    foo(&map);
}
