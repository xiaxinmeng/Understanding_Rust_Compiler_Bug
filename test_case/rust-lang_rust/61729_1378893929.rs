rust
struct Foo {}

impl Foo {
    fn bar() {}
}

fn main() {
    <Foo as Foo>::bar()
}
