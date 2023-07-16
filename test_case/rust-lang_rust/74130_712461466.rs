rust
trait Foo<T> {
    type A;
}
trait Baz {
    type A;
}
trait Qux: Foo<Self::A> + Baz {}
