rust
fn new() -> impl FnOnce(i32) -> impl Future<Output = i32> {
    |x| async move { x }
}
