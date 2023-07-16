
fn foo() {
    let mut set = HashSet::new(); // don't point here!
    set.insert([0_usize; 33]); // <- point here
}
