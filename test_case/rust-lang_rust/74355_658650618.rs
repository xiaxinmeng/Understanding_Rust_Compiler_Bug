rust
#![feature(raw_ref_macros)]

fn main() {
    let mut x = 5;
    // This works.
    let xptr = core::ptr::raw_const!(x);
    // This does not.
    let xptr = core::raw_const!(x);
}
