rust
use std::cell::Cell;

pub const CELL: Option<Cell<u32>> = Some(Cell::new(0));

fn main() {
    let _ = &CELL;
}
