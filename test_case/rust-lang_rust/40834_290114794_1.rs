rust
fn closure_converter<X, Y: ?Sized, F: for<'a> Fn(&'a X) -> &'a Y>(_: F) {}
