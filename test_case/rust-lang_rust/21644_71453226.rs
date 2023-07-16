 rust
fn get_cache_entry(&self, key: &CacheKey) -> &CacheValue {
    match self.cache.entry(key) {
        Entry::Occupied(e) => &*e.into_mut(),
        Entry::Vacant(e) => {
            // do some stuff...
            let cache_value = CacheValue { ... };
            &*e.insert(cache_value)
        }
    }
}
