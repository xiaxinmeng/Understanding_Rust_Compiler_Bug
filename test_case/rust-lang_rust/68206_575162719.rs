rust
use std::cell::Cell;

fn main() {
    let cell = Some(Cell::new(false));
    let cell_ref = &cell;
    if let Some(x) = cell.as_ref() {
        x.set(true);
    }
    // Could this next decision be made with a stale value for cell?
    if let Some(x) = cell_ref.as_ref() {
        x.set(false);
    }
}
