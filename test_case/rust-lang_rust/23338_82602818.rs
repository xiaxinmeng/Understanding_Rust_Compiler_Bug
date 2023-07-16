 rust
use std::cell::RefCell;

fn foo(x: RefCell<String>) -> String {
  x.borrow().clone()
}

fn main() { }
