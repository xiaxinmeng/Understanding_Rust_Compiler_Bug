rust
fn forget_boxed<T: ?Sized>(b: Box<T>) {
    std::mem::forget(*b);
}
