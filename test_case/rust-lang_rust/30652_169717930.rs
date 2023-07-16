 rust
trait Foo {
    type Out;
}

impl Foo for bool {
    type Out = ();
}

impl<T> Foo for T {
    default type Out = bool;
}
