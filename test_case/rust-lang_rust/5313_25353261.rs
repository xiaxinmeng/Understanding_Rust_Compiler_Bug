 rust
fn foo<'a, T>(a: &'a int, _: T) -> &'a int {
    a
}
struct Foo<'self, T> {
    bar: &'self T
}
impl<'self, T> Foo<'self, T> {
    fn new<'a>(a: &'a T) -> Foo<'a, T> {
        Foo { bar: a }
    }
}
fn main() {}
