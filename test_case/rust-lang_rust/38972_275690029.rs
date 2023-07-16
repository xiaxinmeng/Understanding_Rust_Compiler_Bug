rust
#[deny(warnings)]
enum Foo { }
fn make_foo() -> Option<Foo> { None }
fn main() {
    while let Some(_f) = make_foo() { }
}
