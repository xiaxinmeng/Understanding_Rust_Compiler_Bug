 rust
use std::cell::RefCell;

fn main() {
  let b = RefCell::new(Some(5));
  if let Some(x) = b.borrow().clone() {
    println!("x: {}", x);
  }; // <--- note semicolon here, turning the expression into a statement.
}
