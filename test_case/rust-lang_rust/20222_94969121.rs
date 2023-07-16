 rust
trait Foo {}

pub type Bar<T> where T: Foo = T;

fn main() {}
