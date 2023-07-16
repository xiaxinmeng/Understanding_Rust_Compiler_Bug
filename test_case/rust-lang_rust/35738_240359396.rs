 rust
fn range<Min: ?Sized + Ord, Max: ?Sized + Ord>(&self, min: Bound<&Min>, max: Bound<&Max>) -> Range<K, V> where K: Borrow<Min> + Borrow<Max>
