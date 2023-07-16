rust
fn bar<F: Foo + Unpin>(mut foo: F) {
    Pin::new(&mut foo).foo(); // just `foo.foo()` with `&pin` coercion I think...
}

fn baz<F: Foo>(foo: F) {
    stack_pinned(foo, |mut foo: Pin<F>| {
        bar(Pin::borrow(&mut foo));
    }
}
