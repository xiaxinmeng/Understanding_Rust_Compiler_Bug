rust
trait Foo<T> {
    type B;
}
trait Bar<T> {}
trait Baz {
    type A;
}
trait Qux: Foo<Self::A> + Bar<Self::B> + Baz {}
