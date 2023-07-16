
// unsafe_unwrap()
impl<T> UnsafeUnwrap<T> for Option<T> {
    #[inline]
    unsafe fn unsafe_unwrap(self) -> T {
     if let Some(x) = self { x } else { unreachable() }
    }
}
