rust
fn serve<S: for<'a> Service<&'a ()>>() {}
