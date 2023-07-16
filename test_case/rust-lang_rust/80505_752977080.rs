rust
use std::{rc::Rc, mem};

struct A(u8);
#[repr(transparent)]
struct B(A);

trait T {
    fn f(&self);
}
impl T for A {
    fn f(&self) { println!("A") }
}
impl T for B {
    fn f(&self) { println!("B") }
}


fn main() {
    let b = Rc::new(B(A(92)));
    let a = unsafe {
        mem::transmute::<Rc<B>, Rc<A>>(b.clone())
    };
    
    let a: Rc<dyn T> = a;
    let b: Rc<dyn T> = b;
    println!("{}", Rc::ptr_eq(&a, &b));
}
