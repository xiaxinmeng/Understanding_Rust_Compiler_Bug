 rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(0);
    if *x.borrow() == 0 {} else {} // error: `x` does not live long enough
}
