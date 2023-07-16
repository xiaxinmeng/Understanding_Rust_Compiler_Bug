rust
use std::cell::Cell;

thread_local!(static LOCK_HELD: Cell<bool> = Cell::new(false));

fn main() {}
