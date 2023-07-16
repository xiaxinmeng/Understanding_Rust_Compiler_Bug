rust
impl std::hash::Hash for fpreg_t {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.d.to_bits().hash(state);
    }
}
