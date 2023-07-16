 rust
// crate1.rs
trait Foo {}
impl<T,U> Foo for (T,U) {}

// crate2.rs
impl<T> Foo for (T, int) {}

// crate3.rs
impl<U> Foo for (float, U) {}
