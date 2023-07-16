
struct KeyWithCachedHash<K, H: Hasher> {
    hash: u64,
    key: K
}

trait HashMapCacheExt<K, V, H> {
    fn inner_mut(&mut self) -> &mut HashMap<K, V, H>;

    fn insert_cached(&mut self, key: KeyWithCachedHash<T, H>, value: V) -> Option<V> {
        self.inner()..raw_entry_mut().from_key_hashed_nocheck(key.hash, key.key).insert(value)
    }
}
