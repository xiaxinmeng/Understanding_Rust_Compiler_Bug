rust
impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub fn into_key(self) -> &'a K { /* ... */ }
}
