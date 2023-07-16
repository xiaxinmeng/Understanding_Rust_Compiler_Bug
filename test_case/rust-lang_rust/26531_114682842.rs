 rust
// map methods (could be confused with the `Entry` API)
fn get_entry<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>;
fn get_entry_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<(&K, &mut V)>;
fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>;
fn insert_entry(&mut self, key: K, value: V) -> Option<(K, V)>;

// set methods
fn get_item<Q: ?Sized>(&self, item: &Q) -> Option<&T>;
fn remove_item<Q: ?Sized>(&mut self, item: &Q) -> Option<T>;
fn insert_item(&mut self, item: T) -> Option<T>;
