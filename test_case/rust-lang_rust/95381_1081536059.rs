 rust
use std::cell::RefCell;

fn main() {
    let leaf = RefCell::new(());

    if let a = leaf.borrow() {}
}
