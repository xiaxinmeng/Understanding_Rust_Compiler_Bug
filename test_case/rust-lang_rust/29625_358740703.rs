rust
use std::rc::Rc;
fn main() {
    let _: &Fn() = &main;
    let _: Box<Fn()> = Box::new(main);
    let _: Rc<Fn()> = Rc::new(main);
}
