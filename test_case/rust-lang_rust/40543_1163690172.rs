rust
use core::cell::Cell;
pub const TEST_CELL: Cell<u32> = Cell::new(10);

pub fn test(x: &Cell<u32>) -> u32 {
    x.set(x.get() + 1);
    x.get()
}

pub fn main() {
    // 11 is printed twice
    println!("got: {}", test(&TEST_CELL));
    println!("got: {}", test(&TEST_CELL));
}
