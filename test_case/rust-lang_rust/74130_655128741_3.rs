
trait Foo<T> {
    type B;
}
trait Bar<T> {}
trait Baz {
    type A;
}
trait Qux: Foo<<Self as Baz>::A> + Bar<Self::B> + Baz {}
