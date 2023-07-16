 rust
impl<T: ?Sized+Copy+Unsize<U>, U: ?Sized> CoerceUnsized<Rc<U>> for Rc<T> {}
