rust
impl<'a, T> vec::Drain<'a, T> {
    pub fn as_slice(&self) -> &[T];
}

impl<'a, T> AsRef<[T]> for vec::Drain<'a, T> {...}
