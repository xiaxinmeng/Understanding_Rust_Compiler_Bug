 rust
impl<K, V> TreeMap<K, V> {
    fn get<Q>(&self, q: &Q) -> Option<&V> where K: Ord<Q> { /* ... */ }
}

impl<T, U: BorrowFrom<T>> Ord<U> for T { /* ... */ }
