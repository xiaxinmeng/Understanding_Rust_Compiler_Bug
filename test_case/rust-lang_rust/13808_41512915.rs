 rust
struct Foo {
    listener: <'a> ||: 'a
}

impl Foo {
    fn new(listener: <'a> ||: 'a) -> Foo {
        Foo {
            listener: listener
        }
    }
}

fn main() {
    let a = Foo::new();
}
