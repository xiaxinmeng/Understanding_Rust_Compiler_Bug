
#![feature(box_syntax)]
trait Foo: Send + Sync {}

impl Foo for i32 {}

impl Foo for Box<Foo> {}

fn f<F: Foo>(_: F) {}

fn main() {
    let foo: Box<Foo+Send+Sync> = box 10;
    f(foo)
}
