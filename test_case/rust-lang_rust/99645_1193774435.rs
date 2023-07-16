rust
struct A<'a>(&'a ());

trait Foo {}

impl<F: Fn(A)> Foo for F {}

fn main() {
    fn use_foo<F: Foo>(_: F) {}
    use_foo(|_: A<'_>| {});
}
