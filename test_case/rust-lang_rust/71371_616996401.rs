rust
trait Foo {
    fn foo() -> Bar<Self>;
}

struct Bar<T: ?Sized>(T);
