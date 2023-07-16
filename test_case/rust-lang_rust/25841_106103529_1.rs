 rust
use std::cell::RefCell;

fn main() {
    let overflower = RefCell::new(());
    loop {
        std::mem::forget(overflower.borrow());
    }
}
