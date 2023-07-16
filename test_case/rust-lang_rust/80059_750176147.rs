rust
pub struct S {
    x: (),
    y: u32,
}

pub fn f(s: *const S) {
    let S { x: (), .. } = *s;
}

pub fn g(a: *const u8) {
    let 0u8..=255u8 = *a;
}
