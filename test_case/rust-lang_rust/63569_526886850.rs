rust
impl<'a, T> Vec<T> {
    pub fn as_uninit(&'a mut self) -> &'a mut [MaybeUninit<T>] {...}
}
