 rust
trait Foo {
    type Bar: Foo;
}
// This seems fine.
impl Foo for Baz {
    type Bar = Self;
}
// Problematic
impl<T> Foo for Buzz<T> {
    type Bar = Buzz<Self>;
}
