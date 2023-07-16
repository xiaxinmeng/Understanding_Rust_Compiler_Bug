rust
/// Doc comments
impl<Q, K, V> Index<Q> for BTreeMap<K, V> where /*something*/ {
    type Output = V;
    /// More doc comments
    fn index(&self, key: Q) -> &V { ... }
}
