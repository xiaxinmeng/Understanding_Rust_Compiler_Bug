 rust
trait Foo {}
impl<F, A, T> Foo for F where F: Fn(A) -> T {}
