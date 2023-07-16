 rust
struct Foo<T>;

trait Bar {}

impl<T: Bar> Clone for Foo<T> {
    fn clone(&self) -> Foo<T> { Foo }
}

#[deriving(Clone)]
struct Baz<T> {
    x: Foo<T>
}

fn main() {}
