 rust
#![feature(box_syntax)]

trait Foo {}
struct Foobar;
impl Foo for Foobar {}
struct AnyFoo(Box<Foo+'static>);

fn main() {
    AnyFoo(box Foobar as _);
}
