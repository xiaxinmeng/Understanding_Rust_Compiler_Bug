rust
struct A<'a>(&'a ());

trait Foo {}

impl<'a, F: Fn(A<'a>)> Foo for F {}

fn main() {
    fn use_foo<F: Foo>(_: F) {}
    use_foo(|_| {});
}
