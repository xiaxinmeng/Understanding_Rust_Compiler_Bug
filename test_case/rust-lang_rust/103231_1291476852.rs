rust
impl StableHasherResult for u128 {
    #[inline]
    fn finish(hasher: StableHasher) -> Self {
        let (_0, _1) = hasher.finalize();
        u128::from(_0) | (u128::from(_1) << 64)
    }
}

impl StableHasherResult for u64 {
    #[inline]
    fn finish(hasher: StableHasher) -> Self {
        let f = hasher.finalize();
        println!("{} {}", f.0, f.1);
        f.0
    }
}
