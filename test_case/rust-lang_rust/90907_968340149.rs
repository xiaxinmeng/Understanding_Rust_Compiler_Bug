rs
use std::cell::RefCell;

fn foo(x: &RefCell<Box<[u8]>>) {
    let y = x.borrow();
    let z: &[u8] = y.as_ref();

    println!("{:?}", z);
}

fn main() {
    foo(&RefCell::new(Box::new([1, 2, 3_u8])));
}
