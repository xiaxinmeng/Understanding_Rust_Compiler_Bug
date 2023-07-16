rust
trait Foo {
   fn new() -> Self;
}
const fn foo<T: Foo>() -> T {
    T::new()
}
const fn foo2<T: Foo>() {
}
