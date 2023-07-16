Rust
pub struct Foo<T>(T, [u8; 64]);

pub fn abc<'a>(x: &Foo<Box<for<'b> Fn(&'b u8)>>) -> &Foo<Box<Fn(&'a u8)>> { x }

fn main() {
    abc(&Foo(Box::new(|_x| ()), [0; 64]));
}
