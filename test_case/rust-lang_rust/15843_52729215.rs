 rust
trait Foo {
    fn foo();
}

pub fn bar<T: Foo>(data: T) {
    data.foo()
}

fn main() {}
