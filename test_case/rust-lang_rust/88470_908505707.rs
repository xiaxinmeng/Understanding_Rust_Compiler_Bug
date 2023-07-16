rust
#![warn(rust_2021_prelude_collisions)]

pub trait A {
    fn try_from(self) -> i32;
}

struct Foo<'a>(&'a i32);

impl<'a> A for Foo<'a> {
    fn try_from(self) -> i32 {
        0
    }
}

fn main() {
    Foo::try_from(todo!());
}
