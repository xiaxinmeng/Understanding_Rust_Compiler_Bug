rust
trait Foo<T /* not static */> {
    fn foo() -> &'static T;
}
