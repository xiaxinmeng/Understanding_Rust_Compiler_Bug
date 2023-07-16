
// src/libstd/collections/hashmap.rs
pub type HashMap<K, V, H = RandomizedSipHasher> = core_collections::HashMap::<K, V, H>;
