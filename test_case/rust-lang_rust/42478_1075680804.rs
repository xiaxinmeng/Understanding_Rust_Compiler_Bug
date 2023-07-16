rust
use std::rc::Rc;
use std::fmt::Debug;

fn main() {
    // unsize, then convert to Rc
    let b2: Box<dyn Debug> = Box::new((42, 82));
    let p2: Rc<dyn Debug> = Rc::from(b2);
}
