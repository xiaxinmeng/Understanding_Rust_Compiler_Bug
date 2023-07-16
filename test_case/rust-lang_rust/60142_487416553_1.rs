rust
impl OccupiedEntry<'a, K, V> {
    fn key(&self) -> &K {…}
    fn get(&self) -> &V {…}
    fn get_mut(&mut self) -> &mut V {…}
    fn into_mut(self) -> &'a mut V {…}
}