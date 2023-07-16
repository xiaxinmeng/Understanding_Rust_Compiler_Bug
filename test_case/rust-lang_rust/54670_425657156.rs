Rust
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut Pin<&'b mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(&*self)
    }
}
