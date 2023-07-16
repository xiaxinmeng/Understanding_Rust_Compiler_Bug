 rust
impl<'a, T, Sized? U: AsSlice<T>> AsSlice<T> for &'a U {
    #[inline(always)]
    fn as_slice<'a>(&'a self) -> &'a [T] { AsSlice::as_slice(*self) }
}
