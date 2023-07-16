
#![feature(nll)]

use std::cell::RefCell;

fn main() {
    let s = RefCell::new(Some(10));
    if let Some(n) = *(s.borrow_mut()) {
        println!("num: {}", n);
    }
}
