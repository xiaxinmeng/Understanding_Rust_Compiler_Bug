
pub(crate) const MULTIPLE: u64 = 6364136223846793005;

#[inline(always)]
pub(crate) const fn folded_multiply(s: u64, by: u64) -> u64 {
    let result = (s as u128).wrapping_mul(by as u128);
    ((result & 0xffff_ffff_ffff_ffff) as u64) ^ ((result >> 64) as u64)
}

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: u64) {
        self.hash = folded_multiply(i ^ self.hash, MULTIPLE);
    }
}
