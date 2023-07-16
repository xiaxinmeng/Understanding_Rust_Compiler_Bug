rust
impl<'a, T: Foo + ?Sized + 'a> Foo for Pin<'a, T> {
    fn foo(self: Pin<Self>) {
        <T as Foo>::foo(*self)
    }
}

fn bar(foo: Pin<Foo>) {
    foo.foo()
}
