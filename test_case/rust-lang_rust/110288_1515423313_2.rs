rust
use std::cell::Cell;
pub const fn g() {
    // OK:
    let _a: Option<Cell<String>> = None;
    // Error:
    let _b: Option<Cell<String>> = None;
    &_b;
}
