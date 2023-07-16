 rust
pub struct Foo;

impl Foo {
    pub fn shrink(&self) -> Box<Iterator<Item=String>> {
        Box::new(None.into_iter())
    }
}

fn main() {
}
