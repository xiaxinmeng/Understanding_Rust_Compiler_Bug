 rust
struct Foo {
    bar: Bar,
}

// generated
impl Clone for Foo {
    fn clone(&self) -> Foo {
        fn is_clone<C: Clone>() {}
        is_clone::<Bar>();
        *self
    }
}
