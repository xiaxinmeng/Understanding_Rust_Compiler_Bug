 rust
impl<T> ops::Index<uint, T> for [T] {
    fn index(&self, &index: &uint) -> &T {
    assert!(index < self.len());
    unsafe { mem::transmute(self.repr().data.offset(index as int)) }
}
}
