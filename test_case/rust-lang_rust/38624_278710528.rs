Rust
use std::borrow::BorrowMut;

trait T {}

impl T for u8 {}

fn main() {
    let mut r: Box<T> = Box::new(23u8);
    let _: &mut T = r.borrow_mut();
}
