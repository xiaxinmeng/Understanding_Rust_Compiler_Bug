rust
trait Foo {
    const INIT: Self;
}
struct Bar<T: Foo>(T);
impl<T: Foo> Bar<T> {
    const fn new() -> Self {
        Bar(T::INIT)
    }
}
