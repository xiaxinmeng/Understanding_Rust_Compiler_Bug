
struct Foo;

impl Foo {
    pub fn f() {}
}

type Bar = Foo;

fn main() {
    Bar::f();
}
