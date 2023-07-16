 rust
trait MyTrait<T> {
    fn method(&self, t: &T) where T : Eq;
}

struct Foo;
struct Bar; // note that `Bar` does not derive `Eq`

impl MyTrait<Bar> for Foo {
    fn method(&self, t: &Bar);
}
fn main() {}
