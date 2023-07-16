 rust
fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>;
fn find<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>;
