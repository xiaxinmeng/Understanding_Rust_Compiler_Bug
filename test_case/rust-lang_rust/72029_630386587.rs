rust
#![feature(unsized_fn_params)]

#[repr(align(256))]
#[allow(dead_code)]
struct A {
    v: u8,
}

trait Foo {
    fn foo(&self);
}

impl Foo for A {
    fn foo(&self) {
        assert_eq!(self as *const A as usize % 256, 0);
    }
}

fn foo(x: dyn Foo) {
    x.foo()
}

fn main() {
    let x: Box<dyn Foo> = Box::new(A { v: 22 });
    foo(*x);
}
