rust
#![feature(non_lifetime_binders)]

trait Bar<T: ?Sized> {}

trait Foo: for<T> Bar<T> {}

struct Test;

impl<T: ?Sized> Bar<T> for Test {}

impl Foo for Test {}

fn main() {}
