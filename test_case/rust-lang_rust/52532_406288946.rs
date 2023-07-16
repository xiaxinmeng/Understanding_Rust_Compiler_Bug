rust
fn main() {}

struct Bar<'a>(&'a u32);

impl<'a> Bar<'a> {
    fn foo() {
        fn foo<'a>(_: &'a u32) { }
    }
}
