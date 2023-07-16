rs
struct Bar<T> {
    bar: T
}

struct Foo(u8);
impl Foo {
    fn foo() { }
}

fn main() {
    let thing = Bar { bar: Foo };
    thing.bar.0;
}
