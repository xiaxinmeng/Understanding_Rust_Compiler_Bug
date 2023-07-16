rust
#[derive(Debug)]
enum Foo {
    Bar(fn(&())),
}
