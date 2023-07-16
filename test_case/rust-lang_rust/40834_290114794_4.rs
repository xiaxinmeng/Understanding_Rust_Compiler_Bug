rust
struct Foo {}

trait Bar {}

impl Bar for Foo {}

fn foo_to_foo(foo: &Foo) -> &Foo {
    foo
}

fn foo_to_bar(foo: &Foo) -> &(Bar + 'static) {
    foo
}

fn closure_converter<X, Y: ?Sized, F: Fn(&X) -> &Y>(_: F) {}

fn main() {
    closure_converter::<Foo, Foo, _>(foo_to_foo);
    closure_converter::<Foo, Bar, _>(foo_to_bar);
    closure_converter::<Foo, Bar, _>(|x| foo_to_bar(x));
}
