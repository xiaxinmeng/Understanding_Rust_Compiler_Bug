
pub trait Foo: Iterator<Item=Self::Key> {
    type Key;
}

fn main() {}
