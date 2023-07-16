rust
#![feature(generic_associated_types)]
trait Foo { type Bar<T>; }
struct Lol;
impl Foo for Lol { type Bar<T> = T; }
