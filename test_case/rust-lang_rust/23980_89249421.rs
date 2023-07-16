 rust
// crate A
trait Foo { }
impl<T> Foo for &T { }

// crate B
struct MyType;
impl Foo for &MyType { } // X
