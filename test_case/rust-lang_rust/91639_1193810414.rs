rust
struct A<'a>(&'a ());
trait Foo {}

impl<F: FnOnce(A)> Foo for F {}
