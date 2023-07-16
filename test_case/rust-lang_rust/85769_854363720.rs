rust
const fn alpha(x: u32) -> (u16, u16) {
    unsafe { core::mem::transmute(x) }
}

union Beta {
    a: u32,
    b: (u16, u16),
}

// This is effectively a transmute, but that's not the point.
const fn beta(x: u32) -> (u16, u16) {
    unsafe { Beta { a: x }.b }
}
