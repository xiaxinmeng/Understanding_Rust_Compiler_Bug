rust
fn capacity(&self) -> usize {
    cmp::max(self.vec.capacity(), self.vec.len())
}
