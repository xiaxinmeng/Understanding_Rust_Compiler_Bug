rust
pub struct Foo<T>(T);

pub struct Bazzzz<T>(T);

pub trait Baz {}
impl<T> Baz for Bazzzz<T> {}

impl<T> Foo<T> {
    fn baz(self) -> Box<dyn Baz> {
        Box::new(Bazzzz(self.0))
    }
}

fn main() {}
