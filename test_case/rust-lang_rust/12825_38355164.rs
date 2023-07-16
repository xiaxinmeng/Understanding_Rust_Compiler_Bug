 rust
use std::cell::RefCell;

fn main() {
    let a = RefCell::new((1i, 2i));
    match *a.borrow_mut() {
        (ref mut a, ref mut b) => { *a += *b; *b += *a; }
    }
}
