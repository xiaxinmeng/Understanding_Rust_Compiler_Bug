rust
#![warn(rust_2021_prelude_collisions)]

fn bar() {}

macro_rules! foo {
    () => {{
        $crate::bar();
        S
    }};
}

trait MyTry<T> {
    fn try_into(self, _: u8);
}

struct S;

impl MyTry<i32> for S {
    fn try_into(self, _: u8) {}
}

fn main() {
    foo!().try_into(todo!());
}
