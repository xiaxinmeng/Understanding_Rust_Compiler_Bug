Rust
use std::fmt;
fn main() {
    let foo: *mut (fmt::Display,) = 0 as *mut _;
}
