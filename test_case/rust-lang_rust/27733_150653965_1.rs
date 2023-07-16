 rust
use std::cell::BorrowState;
...
if BorrowState::None == cell.borrow_state() {
    let borrow = cell.borrow_mut();
    ..
}
