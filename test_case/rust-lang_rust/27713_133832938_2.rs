 rust
fn hash_one_shot<H: Hasher>(&self, state: &mut H) -> u64 {
    state.write_only(self.as_slice())
}
