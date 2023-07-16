rust
struct Foo;

impl Iterator for Foo {
    type Item = Foo;

    fn next(&mut self) -> Foo {
        unimplemented!()
    }
}
