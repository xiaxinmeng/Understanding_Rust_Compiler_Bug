 rust
trait MakeHasher {
    type Hasher;
    fn make_hasher(&self) -> Hasher;
}

impl HashMap<K, V, H: MakeHasher> {
    fn with_hasher(hasher: H) { ... }
    fn with_capacity_and_hasher(hasher: H) { ... }
}
