rust
impl<'a, T> slice::IterMut<'a, T> {
    pub fn as_slice(&self) -> &[T];
}

impl<'a, T> vec::Drain<'a, T> {
    pub fn as_slice(&self) -> &[T];
}
