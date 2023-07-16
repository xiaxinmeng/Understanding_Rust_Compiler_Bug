 rust
impl<K: Unsigned, T> ops::Index<K, T> for [T] {
    fn index(&self, &index: &K) -> &T {
    assert!(index < self.len());
    unsafe { mem::transmute(self.repr().data.offset(index as int)) }
}
}
