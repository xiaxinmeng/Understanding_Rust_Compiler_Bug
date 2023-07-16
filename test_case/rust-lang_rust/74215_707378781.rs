rust
pub enum Foo {
    Bar = 12345,
    Baz = 1 + Foo::Bar as isize,
}
