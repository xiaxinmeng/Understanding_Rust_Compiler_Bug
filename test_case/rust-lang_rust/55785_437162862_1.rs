rust
fn drop<T: ?Sized>(_: T) {}
fn drop_boxed<T: ?Sized>(b: Box<T>) {
    drop(*b);
}
