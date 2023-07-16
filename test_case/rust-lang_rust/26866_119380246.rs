 rust
pub struct Foo;

impl Foo {
    pub fn shrink(&self) -> Box<Iterator<Item=Foo>> {
        Box::new(None.into_iter())
    }
}

fn main() {
}
