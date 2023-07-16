rust
#[derive(Eq)]
pub struct Foo(u32);

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= 3
    }
}

pub fn foo(x: Foo) {
    const FOUR: Foo = Foo(4);
    if let FOUR = x {}
}
