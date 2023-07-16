rust
// lib.rs
pub extern "C" fn yep() -> bool {
    true
}

// main.rs
#![feature(start)]
#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    return 0;
}
