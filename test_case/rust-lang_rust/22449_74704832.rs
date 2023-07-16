 rust
use std::cell::{RefCell, BorrowState};

fn main() {
  let b = RefCell::new(Some(5));
  if let Some(x) = b.borrow().clone() {
    assert_eq!(b.borrow_state(), BorrowState::Reading);
  };
}
