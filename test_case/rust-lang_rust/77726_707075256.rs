rust
impl<T: ?Sized> Pin<&'static T> {
    pub fn static_ref(r: &'static T) -> Self { ... }
}

impl<T: ?Sized> Pin<&'static mut T> {
    pub fn static_mut(r: &'static mut T) -> Self { ... }
}
