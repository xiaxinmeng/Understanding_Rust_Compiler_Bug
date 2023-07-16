
impl<T: ?Sized, U: ?Sized> AsRef<U> for &T where T: AsRef<U>
{
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}
