rust
trait Foo {}
fn requires_foo<T: Foo>() {}
    
trait Bar {}
impl<T: Bar> Foo for T {}

fn main() {
    // try to call `requires_foo` with a type that doesn't implement it
    requires_foo::<()>();
}
