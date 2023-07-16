rust
// Crate A
trait Foo = Send + Sync;

// Crate b:
use crate_a::Foo;
fn use_b<T: Foo>() { }
fn main() {
    use_b::<u32>(); // Expect Ok
    use_b::<Rc<u32>>(); // Expect error
}
