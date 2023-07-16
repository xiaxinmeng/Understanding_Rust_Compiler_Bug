 rust
trait Foo {
    fn foo(self);
}

impl<T> Foo for [T] {
    fn foo(self) { unimplemented!() }
    //~^ ERROR the trait `core::marker::Sized` is not implemented for the type `[T]`
}
