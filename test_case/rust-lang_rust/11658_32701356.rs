 rust
struct Foo<T>(T);

impl<T> Foo<T> {
     fn new<T>(x: T) -> Foo<T> { Foo(x) }
}

fn main() {
     Foo::new(1i);
}
