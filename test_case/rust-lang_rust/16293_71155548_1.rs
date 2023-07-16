 rust
// std::collections (the collections crate wouldn't have a clue about RNGs):
type HashMap<K, V, H = RandomizedSipHash> = collections_crate::HashMap<K, V, H>;
