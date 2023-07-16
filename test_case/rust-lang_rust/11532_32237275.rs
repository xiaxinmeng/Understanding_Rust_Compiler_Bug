 rust
fn main()   {
    use std::{cell,gc,rc};
    let a = rc::Rc::new(cell::RefCell::new( gc::Gc::new(1) ));
    let x = a.borrow();
    println!("{:?}", x);
    let y = x.try_borrow_mut();
    println!("{:?}", y);
    assert!(y.is_some());
}
