
pub mod A {
    struct Foo;
    pub use A::Foo as Bar;
}

fn main() {
    let b = A::Bar;
}
