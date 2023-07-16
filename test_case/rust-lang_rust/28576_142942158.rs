 Rust
pub trait Foo<T> {
    type Assoc;
}

pub trait Bar: Foo<Self> {
    fn new(&self, b: &Bar<Assoc=()>);
}

fn main() {}
