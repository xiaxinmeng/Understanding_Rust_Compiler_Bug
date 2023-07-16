Rust
impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut Pin<&'long mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(&*self)
    }
}
