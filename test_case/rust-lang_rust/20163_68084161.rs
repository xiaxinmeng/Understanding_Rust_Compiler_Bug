 rust
impl<'a, Q: ToOwned<K>, K: Ord, V> VacantEntry<'a, Q, K, V> {
    /// Sets the value of the entry with the VacantEntry's key,
    /// and returns a mutable reference to it.
    pub fn set(self, value: V) -> &'a mut V {
        self.stack.insert(self.key.to_owned(), value)
    }
}
