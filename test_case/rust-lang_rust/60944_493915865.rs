rust
#![allow(dead_code)]
#![feature(arbitrary_self_types)]
struct Foo;
impl Foo {
    fn b(self: &Box<Foo>, f: &Foo) -> &Foo { f }
}
