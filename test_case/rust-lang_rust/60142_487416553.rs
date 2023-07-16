rust
impl Entry<'a, K, V> {
    fn insert(self, value: V) -> OccupiedEntry<'a, K, V> {…}
}
