 rust
trait Foo {
    fn foo();
}

struct A;
impl Foo for A {
    fn foo() {}
}

pub fn bar<T: Foo>(data: T) {
    data.foo()
}

fn main() {}
