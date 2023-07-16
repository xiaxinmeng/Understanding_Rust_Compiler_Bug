rust
fn walk_source_chain(error: &impl Error) {
    for e in iter::successors(error.source(), |&e| e.source()) {
        println!("{}", e);
    }
}
