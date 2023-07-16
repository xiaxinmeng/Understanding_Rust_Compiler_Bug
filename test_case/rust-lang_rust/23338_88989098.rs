 rust
use std::cell::RefCell;

fn foo(x: RefCell<String>) -> String {
    let result = {
        let t = x;
        t.borrow().clone()
    };
    result
}

fn main() { }
