 rust
use std::default::Default;

trait Maker<Item> {
    fn make(&mut self) -> Item;
}

struct Foo<T> {
    a: T,
}

struct Bar;

impl<T> Maker<Foo<T>> for Bar
    where T: Default  
{

    fn make(&mut self) -> Foo<T> {
        Foo {
            a: <T as Default>::default(),
        }
    }
}
