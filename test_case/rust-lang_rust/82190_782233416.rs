rust
#![feature(raw_ref_op)]

trait Foo {
}

impl Foo for i32 {
}

trait Bar: Sized {
    fn bar(self) {}
}

impl Bar for *const dyn Foo {
    fn bar(self) {}
}

fn main() {
    let x = 42_i32;
    let y = &raw const x;
    y.bar();
    let z: *const dyn Foo = y;
    z.bar();
}
