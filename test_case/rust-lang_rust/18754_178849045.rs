 rust
#![feature(unboxed_closures)]
#![feature(default_type_params)]

pub trait Invoke<A, R = ()> {
    fn invoke(&mut self, args: A) -> R;
}

impl<A, R, F: Send + FnMut(A) -> R> Invoke<A, R> for F {
    fn invoke(&mut self, args: A) -> R {
        self.call_mut((args,))
    }
}

pub struct Foo;


impl Invoke<usize, usize> for Foo {
    fn invoke(&mut self, args: usize) -> usize {
        args
    }
}

fn repro<A, I: Invoke<A>>(_: I) {
    unimplemented!();
}

pub fn main() {
    repro(Foo);
}
