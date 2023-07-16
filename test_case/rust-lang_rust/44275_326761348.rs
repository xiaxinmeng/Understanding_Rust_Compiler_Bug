rust
impl StableHasherResult for [u8; 20] {
    fn finish(mut hasher: StableHasher<Self>) -> Self {
        ...
    }
}
