rust
#![feature(specialization)]
trait Foo {}
default impl<T, U> Foo for (T, U) {}
impl<T> Foo for (T, i32) {}
