rust
pub trait Foo<T> {
    fn foo() -> &'static fn(T);
}
