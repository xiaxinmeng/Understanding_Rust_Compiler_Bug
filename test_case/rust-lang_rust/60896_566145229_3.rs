rust
fn get_or_insert_with<Q: ?Sized, F, E>(&mut self, value: &Q, f: F) -> Result<&T, E>
where
    T: Borrow<Q>,
    Q: Hash + Eq,
    F: FnOnce(&Q) -> Result<T, E>
