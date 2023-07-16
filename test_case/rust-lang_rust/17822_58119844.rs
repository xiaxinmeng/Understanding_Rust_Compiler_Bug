 rust
struct Foo<'a> {...}
impl<'a> Foo<'a> {
    fn bar<'b>(&'b self) -> Bar<'b> {...}
}
