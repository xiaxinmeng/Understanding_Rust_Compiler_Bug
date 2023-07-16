rust
use std::rc::Rc;
use std::fmt::Debug;

fn main() {
    // convert to Rc, then unsize (OK)
    let b1 = Box::new((42, 82));
    let p1 : Rc<Debug> = Rc::new(*b1);

    // unsize, then convert to Rc (fails)
    let b2 : Box<Debug> = Box::new((42, 82));
    let p2 = Rc::new(*b2); // Error
}
