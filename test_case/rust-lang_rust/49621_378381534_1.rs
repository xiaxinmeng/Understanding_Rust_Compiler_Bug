rust
impl<'a, T: Foo + ?Sized + 'a> Foo for Pin<'a, T> {
    fn foo(mut self: Pin<Self>) {
        <T as Foo>::foo(Pin::borrow(unsafe { Pin::get_mut(&mut self) }))
        // <T as Foo>::foo(Pin::borrow(&mut *self))
    }
}
