 rust
impl<T: ?Sized, V: AsRef<T> + ?Sized> AsRef<T> for Box<V> {
    #[inline]
    fn as_ref(&self) -> &T {
        (&*self).as_ref()
    }
}
