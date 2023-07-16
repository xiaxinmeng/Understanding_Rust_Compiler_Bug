`rust
trait Foo {
    type V;
}

trait Callback<T: Foo>: Fn(&Bar<T>, &T::V) {}

struct Bar<'a, T> {
    callback: Box<dyn Callback<dyn Callback<Bar<T>>>>,
}

impl<T: Foo> Bar<Bar<T>> {}

fn main() {}
