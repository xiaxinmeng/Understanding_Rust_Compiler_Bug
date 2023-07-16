rust
fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    Foo::fun(&|_| impl_foo(x))(PhantomData)
}
