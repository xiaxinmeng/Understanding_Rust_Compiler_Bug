 rust
struct Foo;
trait Bar<T> {
    fn forget(&self, _: T) {}
}
impl<T> Bar<T> for Foo {}
fn main() {
    let foo = Foo;
    foo.forget(0i8);
}
