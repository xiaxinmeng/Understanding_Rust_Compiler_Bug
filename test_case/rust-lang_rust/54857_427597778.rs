Rust
let x: &() = &*(0x80000000 as *const u8);
let y: [u8; 2] = 0;
let _t = *x; // force `x` to be valid.

let x_addr = x as *const () as usize;
let y_addr = (&y) as *const () as usize;
// `y` is a stack object, and `x` is a valid object, therefore `x` and `y` must be disjoint.
if x_addr.wrapping_sub(y_addr) >= mem::size_of::<[u8; 2]>() {
    something_involving(x_addr, y_addr);
}
