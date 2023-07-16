rust
impl<'a, T: ?Sized> Pin<&'a T> {
    fn map<U: Unpin, F: FnOnce(&T) -> &U>(self, f: F) -> Pin<&'a U>
}

impl<'a, T: ?Sized> Pin<&'a mut T> {
    fn map<U: Unpin, F: FnOnce(&mut T) -> &mut U>(self, f: F) -> Pin<&'a mut U>
}
