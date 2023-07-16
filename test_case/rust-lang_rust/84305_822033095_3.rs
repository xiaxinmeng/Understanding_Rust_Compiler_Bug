rust
fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    let y = impl_foo(x);
    Foo::fun(&|_| y)(PhantomData)
}
