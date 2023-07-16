 rust

use std::rc::Rc;

trait A {
    fn blub(&self) { println!("blub") }
}

struct X;

impl A for X {
    fn blub(&self) { println!("X") }
}

impl<T: A + ?Sized> A for Box<T> {
    fn blub(&self) { (**self).blub() }
}

fn main() {
    let x = Box::new(X);
    let y: Box<A> = x;
    let z: Rc<A> = Rc::new(y);
    z.blub();
}
