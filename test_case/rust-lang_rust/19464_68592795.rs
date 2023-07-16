 rust
#![feature(associated_types)]

pub trait Foo {
    type A;
    fn boo(&self) -> <Self as Foo>::A;
}

struct Bar;

impl Foo for int {
    type A = uint;
    fn boo(&self) -> uint {
        42
    }
}

fn foo1<I: Foo<A=Bar>>(x: I) {
    let _: Bar = x.boo();
}

fn foo2<I: Foo>(x: I) {
    let _: Bar = x.boo();
}


pub fn baz(x: &Foo<A=Bar>) {
    let _: Bar = x.boo();
}


pub fn main() {
    let a = 42i;
    foo1(a);
    baz(&a);
}
