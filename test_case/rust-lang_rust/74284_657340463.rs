Rust
impl PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        *self.child == *other.child
    }
}
