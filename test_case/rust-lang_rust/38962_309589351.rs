Rust
use std::borrow::Cow;

// Does not work either: #[derive(Clone)]
enum Foo<'a> {
    // Does not work:
    Bar(Cow<'a, [Foo<'static>]>, ()),
    // Works: Bar(Vec<Foo<'a>>),
    Baz(&'a str),
}

impl<'a> Clone for Foo<'a> {
    fn clone(&self) -> Self {
        unimplemented!();
    }
}

fn main() {}
