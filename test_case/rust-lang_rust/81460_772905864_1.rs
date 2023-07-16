rust
fn walk_source_chain(error: &(dyn Error + 'static)) {
    for e in error.chain() {
        println!("{}", e);
    }
}
