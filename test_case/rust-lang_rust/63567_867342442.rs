rust
impl<T> MaybeUninit<T> {
    pub fn write(&mut self, value: T) -> &mut T
}
