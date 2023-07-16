 rust
pub unsafe fn a() -> u8 {
    let mut x = 11;
    b(&x as *const u8 as *mut u8);
    x
}

unsafe fn b(x: *mut u8) {
    *x = 22;
}
