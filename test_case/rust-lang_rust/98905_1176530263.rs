
// As lifts over &
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_convert", issue = "88674")]
impl<T: ?Sized, U: ?Sized> const AsRef<U> for &T
where
    T: ~const AsRef<U>,
{
    #[inline]
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}
