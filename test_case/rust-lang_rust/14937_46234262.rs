 rust
use std::cell::RefCell;
fn main() {
    let b = {
        let a = box RefCell::new(4);
        *a.borrow() + 1
    };
    println!("{}", b);
}
