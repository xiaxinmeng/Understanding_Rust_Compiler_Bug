 rust
fn get_cache_entry(&self, key: &CacheKey) -> &CacheValue {
    match self.cache.get(key) {
        Option::Some(entry) => entry,
        Option::None => {
            // do some stuff...
            let cache_value = CacheValue {
                texture: texture,
                glyph: glyph,
                bitmap_glyph: bitmap_glyph,
            };
            self.cache.entry(*key).get().unwrap_or_else(|e| e.insert(cache_value))
        },
    };
}
