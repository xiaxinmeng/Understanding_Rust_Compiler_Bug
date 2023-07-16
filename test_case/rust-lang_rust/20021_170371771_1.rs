 rust
trait Foo {
    fn foo(self) where Self: Sized { unimplemented!() }
}

impl<T> Foo for [T] {}
