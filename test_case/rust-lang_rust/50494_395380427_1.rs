rust
// Ideally this would use variadic generics.
impl<A, B> Deref for Cell<(A, B)> {
    type Target = (Cell<A>, Cell<B>);
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}
impl<A, B> DerefMut for Cell<(A, B)> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}
