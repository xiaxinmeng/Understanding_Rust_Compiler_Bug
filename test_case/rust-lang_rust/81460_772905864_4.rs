rust
fn dyn_walk_source_chain<'a>(error: &'a &'a (dyn Error + 'static)) {
    for e in error.chain() { // <-- would now fail to assert that `&'a (dyn Error + 'static)` is `'static`
        println!("{}", e);
    }
}
