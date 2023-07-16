 rust
use std::cell::RefCell;
fn main() {
    let cell = RefCell::new(1);
    println!("{:?}", cell);
    let _b = cell.borrow_mut();
    println!("{:?}", cell);
}
