 rust
struct Foo<'a> {
    listener: ||: 'a
}

impl<'a> Foo<'a> {
    fn new(listener: ||: 'a) -> Foo<'a> {
        Foo { listener: listener }
    }
}

fn main() {
    let a = Foo::new(|| {});
}
