rust
#[repr(transparent)]
struct Foo<T> {
    inner: T,
}

extern "C" {
    fn foo(v: Foo<()>);
}
