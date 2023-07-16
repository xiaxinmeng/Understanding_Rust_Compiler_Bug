rust
fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}
