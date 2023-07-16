rust
trait Foo {
    type Out;
    fn method() -> Self::Out;
}

struct Bar<T>(T)

impl<A, B: Foo<Out = A>> Bar<B> {
}
