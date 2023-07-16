rust
fn foo<F: FnOnce() -> R, R: ?Sized>() {}
