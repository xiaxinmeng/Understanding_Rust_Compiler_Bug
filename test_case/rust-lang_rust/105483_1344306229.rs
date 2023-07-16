rust
trait Trait {
    fn get(_: &'static &'static ());
}
impl<'a, 'b> Trait for &'a &'b () {
    fn get(_: &'a &'b ()) {}
}
