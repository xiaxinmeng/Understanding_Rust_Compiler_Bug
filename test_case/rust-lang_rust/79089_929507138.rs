rust
use std::os::raw::c_char;
fn main() {
    let x: u8 = 0b10000000; // set the last bit. 128 in decimal
    let x = x as i8 as c_char;
    let x = x >> 1; // a non-portable operation
}
