
pub mod A {
    pub struct Foo;
    pub use A::Foo as Bar;
}

fn main() {
    let b: i32 = A::Bar;
}
