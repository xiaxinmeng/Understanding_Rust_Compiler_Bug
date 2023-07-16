
#![feature(associated_consts)]

trait Foo {
    type Associated;
}

trait FooConst: Foo {
    const ID: <Self as Foo>::Associated;
}

impl Foo for i32 {
    type Associated = f32;
}

impl FooConst for i32 {
    const ID: f32 = 1f32;
}

fn main() {
    assert_eq!(1f32, i32::ID);
}
