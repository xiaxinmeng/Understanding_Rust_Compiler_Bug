
impl<T> UnsafeUnwrap<T> for Option<T> {
    #[inline]
    unsafe fn unsafe_unwrap(self) -> T {
     //if let Some(x) = self { x } else { unreachable() }
     if let Some(x) = self { x } else { panic!("PANIC") }
    }
}
