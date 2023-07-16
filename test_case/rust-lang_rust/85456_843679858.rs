rust
trait Query {
    fn query();
}
impl<'a> Query for TemporaryStruct<'a> {
    fn query() {
        println!("TemporaryStruct queried");
    }
}
fn evil<'a>(_: &'a Box<i32>) -> Box<dyn Fn()> {
    Box::new(TemporaryStruct::<'a>::query)
}
