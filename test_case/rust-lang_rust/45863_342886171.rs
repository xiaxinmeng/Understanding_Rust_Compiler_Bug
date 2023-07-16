rust
fn filter<P>(self, predicate: P) -> Self
where
    P: FnOnce(&T) -> bool
