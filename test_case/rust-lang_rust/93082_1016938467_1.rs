rust
fn old() -> impl for<'a> FnOnce(&'a i32) -> Box<dyn Future<Output = &'a i32> + 'a> {
    |x| Box::new(async move { x })
}
