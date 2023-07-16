rust
fn get_or_insert_with<Q: ?Sized, F>(&mut self, value: &Q, f: F) -> Option<&T>
where
    T: Borrow<Q>,
    Q: Hash + Eq,
    F: FnOnce(&Q) -> Option<T>
