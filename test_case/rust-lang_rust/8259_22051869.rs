 rust
// test.rs
extern mod other;
static a: other::Bar<'static> = other::C(&other::A);

fn main() {}

// other.rs
pub enum Foo<'self> {
    A,
    B(&'self str),
}
pub enum Bar<'self> {
    C(&'self Foo<'self>),
}
