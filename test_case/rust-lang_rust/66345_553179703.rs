rust
fn do_nothing(_: &()) {}

pub struct Foo {
    bar: (),
}

pub fn by_value<T>(foo: Foo) {
    do_nothing(&foo.bar);
}
