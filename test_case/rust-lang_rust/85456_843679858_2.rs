rust
fn evil<'a>(_: &'a Box<i32>) -> Box<fn()> {
    Box::new(TemporaryStruct::<'a>::query)
}
