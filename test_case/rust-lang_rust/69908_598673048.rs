rust
use std::cell::Cell;
const FOO: () = {
    let x = Cell::new(42);
    let y = &x;
};
