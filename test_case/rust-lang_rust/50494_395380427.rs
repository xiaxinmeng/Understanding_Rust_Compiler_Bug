rust
impl<T, const N: usize> Deref for Cell<[T; N]> {
    type Target = [Cell<T>; N];
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}
impl<T, const N: usize> DerefMut for Cell<[T; N]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}
