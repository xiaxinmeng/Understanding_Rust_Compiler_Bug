rust
impl<T, U: ?Sized> CoerceUnsized<Foo<U>> for Foo<T> where T: Unsize<U> {}
