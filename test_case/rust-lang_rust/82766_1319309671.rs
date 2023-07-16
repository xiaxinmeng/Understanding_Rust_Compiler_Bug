rust
impl<K: Eq + Hash, V, S> HashMap<K, V, S> {
    pub fn get_or<Q: ?Sized>(&mut self, k: &Q, v: V) -> Option<&V>where K: Borrow<Q>, Q: Hash + Eq { ... }
    pub fn get_mut_or<Q: ?Sized>(&mut self, k: &Q, v: V) -> Option<&mut V>where K: Borrow<Q>, Q: Hash + Eq { 
        if let Some(v) = self.get(k) {
            v
        } else {
            self.entry(k.to_owned()).or_insert(v)
        }
    }
    pub fn get_or_with<Q: ?Sized, F: FnOnce() -> V>(&mut self, k: &Q, f: F) -> Option<&V>where K: Borrow<Q>, Q: Hash + Eq { ... }
    pub fn get_mut_or_with<Q: ?Sized, F: FnOnce() -> V>(&mut self, k: &Q, f: F) -> Option<&mut V>where K: Borrow<Q>, Q: Hash + Eq { ... }
impl<K: Eq + Hash, V: Default, S> HashMap<K, V, S> {
    pub fn get_or_default<Q: ?Sized>(&mut self, k: &Q) -> Option<&V>where K: Borrow<Q>, Q: Hash + Eq { ... }
    pub fn get_mut_or_default<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>where K: Borrow<Q>, Q: Hash + Eq { ... }
}
