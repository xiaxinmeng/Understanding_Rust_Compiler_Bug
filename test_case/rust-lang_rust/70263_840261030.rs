rust
fn foo<F>(f: F) where F: for<'a> MyFn<'a> {}
