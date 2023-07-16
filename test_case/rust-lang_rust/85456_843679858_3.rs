rust
fn evil<'a>(_: &'a Box<i32>) -> Box<dyn Fn()> {
    Box::new(|| TemporaryStruct::<'a>::query())
}
