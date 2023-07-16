rust
impl<K: Ord, V> BTreeMap<K, V> {
    pub fn range_mut<T: ?Sized, R>(&mut self, range: R) -> RangeMut<'_, K, V>
        where T: Ord, K: Borrow<T>, R: RangeBounds<T>
}
