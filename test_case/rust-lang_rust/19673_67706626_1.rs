 rust
impl Hash<uint> for MyType {
    fn hash(&self, state: &mut uint) {
        *state = self.my_hash();
    }
}
