 rust
fn request<R, S> (rx: Chan<R>) where (R, S): Dual {
    borrow_request::<R, S>(&rx)
}
