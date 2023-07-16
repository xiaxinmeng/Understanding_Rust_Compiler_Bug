 rust
impl<Sized? T> Deref<T> for Box<T> {
    fn deref(&self) -> &T { &**self }
}
