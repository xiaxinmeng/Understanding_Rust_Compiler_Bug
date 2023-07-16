 rust
#![feature(core)]
use std::marker::{MarkerTrait, PhantomFn};

pub trait Wrapper {
    type Inner;
}

pub trait IsA<T>: MarkerTrait + PhantomFn<T> { }

impl <T> IsA<T> for T { }

pub trait ToPtr<P: Copy> {
    fn borrow(&self) -> P;
}

impl <PAR: Wrapper, CHI: IsA<PAR>> ToPtr<*mut <PAR as Wrapper>::Inner> for CHI {
    fn borrow(&self) -> *mut <PAR as Wrapper>::Inner {
        unimplemented!()
    }
}

pub struct Object;
pub struct Window;

impl Wrapper for Window {
    type Inner = i8;
}

impl IsA<Object> for Window { }

impl Window {
    pub fn new() -> Window {
        unimplemented!()
    }
}

pub trait WindowTrait {
    fn bar(&self);
}

impl <T: Wrapper> WindowTrait for T {
    fn bar(&self) {
        ToPtr::<_>::borrow(self);
    }
}

fn main() {
    let window = Window::new();
    window.bar();
}
