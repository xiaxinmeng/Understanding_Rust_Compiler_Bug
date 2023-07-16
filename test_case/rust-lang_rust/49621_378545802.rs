rust
fn bar<F: Foo>(foo: F) {
    stack_pinned(foo, |mut foo: Pin<F>| { // When invoked from `baz2` this is `Pin<Pin<baz::F>>`
        Pin::borrow(&mut foo).foo();
    });
}

fn baz1<F: Foo>(foo: F) {
    bar(foo);
    bar(foo); // Error: use of moved value
}

fn baz2<F: Foo>(foo: F) {
    stack_pinned(foo, |mut foo: Pin<F>| {
        bar(Pin::borrow(&mut foo));
        bar(Pin::borrow(&mut foo)); // just `bar(foo);` with re-borrowing support
    }
}
