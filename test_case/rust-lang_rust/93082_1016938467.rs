rust
fn new() -> impl for<'a> FnOnce(&'a i32) -> (impl Future<Output = &'a i32> + 'a) {
    |x| async move { x }
}
