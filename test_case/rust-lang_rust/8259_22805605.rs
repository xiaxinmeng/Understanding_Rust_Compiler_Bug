
// test.rs
extern mod other;
static a: other::Foo<'static> = other::A(());

fn main() {}

// other.rs
pub enum Foo<'self> {
    A(()),
    B(&'self str),
}

