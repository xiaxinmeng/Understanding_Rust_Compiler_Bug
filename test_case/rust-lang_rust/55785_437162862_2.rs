rust
fn drop_boxed<T: ?Sized>(b: Box<T>) {
    {*b};
}
