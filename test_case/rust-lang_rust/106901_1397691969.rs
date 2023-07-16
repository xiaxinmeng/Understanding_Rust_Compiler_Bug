rust
impl<T: Hash> Hash for std::collections::hash_set::HashSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sum = 0_u64;
        for item in self {
            let mut hasher = DefaultHasher::new();
            item.hash(&mut hasher);
            sum = sum.wrapping_add(hasher.finish());
        }
        state.write_u64(sum);
    }
}
