rust
#![feature(unsized_locals)]
#![feature(unsized_fn_params)]

use std::any::Any;

#[repr(align(256))]
#[allow(dead_code)]
struct A {
    v: u8,
}

impl A {
    fn f(&self) -> *const A {
        assert_eq!(self as *const A as usize % 256, 0);
        self
    }
}

fn foo(x: Box<dyn Any>) {
    let x = *x;
    let dwncst = x.downcast_ref::<A>().unwrap();
    let addr = dwncst.f();
    assert_eq!(addr as usize % 256, 0);
}

fn main() {
    let x = Box::new(A { v: 4 });
    foo(x);
}
