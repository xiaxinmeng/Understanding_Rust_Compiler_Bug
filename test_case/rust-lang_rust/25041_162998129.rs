 rust
trait Foo {}

impl<F, A> Foo for F
    where F: Fn(A),
          A: Foo,
{}
