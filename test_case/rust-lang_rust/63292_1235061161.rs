rs
#![feature(get_mut_unchecked)]
use std::rc::Rc;

fn main() {
    let a: Rc<str> = "Hello, world!".into();
    let mut b: Rc<[u8]> = a.clone().into();
    unsafe {
        Rc::get_mut_unchecked(&mut b).fill(0xc0); // any non-ascii byte
    }
    dbg!(&a);
}
