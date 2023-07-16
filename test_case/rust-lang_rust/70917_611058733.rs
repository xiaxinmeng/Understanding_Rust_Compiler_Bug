rust
fn assert_static<T: 'static>(_: T) {}

fn capture_type<T>() {}
fn capture_lifetime<'a: 'a>() {}

// Errors on stable & nightly.
fn test_type<'a>() {
    assert_static(capture_type::<&'a ()>);
}

// Errors *only* on nightly.
fn test_lifetime<'a>() {
    assert_static(capture_lifetime::<'a>);
}
