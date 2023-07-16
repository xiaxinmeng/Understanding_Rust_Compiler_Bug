 rust
#[derive(Clone)]
pub struct Foo<'a, T: 'a>(&'a T);

pub struct Bar;

fn main() {
    let foo = Foo(&Bar);
    foo.clone();
}
