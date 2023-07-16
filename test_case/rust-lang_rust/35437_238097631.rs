 rust
#![feature(start)]

use std::ptr;

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    let x = unsafe {
        // to prevent the compiler from optimizing away the whole floating point operation
        ptr::read_volatile(0x0 as *const f64)
    };
    let y = 1.;
    let z = x + y;

    z as isize
}
