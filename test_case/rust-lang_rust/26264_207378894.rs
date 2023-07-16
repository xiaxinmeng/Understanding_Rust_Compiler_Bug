 rust
#[derive(Debug)]
pub enum Foo<T> { Bar(T), Baz }
pub type Loz = Foo<i32>;

pub fn qux() -> Loz {
    Loz::Bar(0) // error
}

fn main() {
    println!("{:?}", qux());
}
