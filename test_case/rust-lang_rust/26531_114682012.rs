 rust
fn get_mut<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &mut V)>;
