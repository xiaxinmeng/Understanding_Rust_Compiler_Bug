rust
#![crate_type = "cdylib"]

pub struct V4(u16, u16, u16, u16);
pub struct V8(u16, u16, u16, u16, u16, u16, u16, u16);

#[inline(never)]
fn inv4(v: &V4) -> u16 {
    v.2 ^ (v.2 & v.0)
}

fn split8(a: u16, b: u16, c: u16) -> (V4, V4) {
    (V4(0, 0, 0, a), V4(b, c, 0, 0))
}

#[inline(never)]
fn inv8(v: V8) -> u16 {
    let (_b, a) = split8(v.3, v.4, v.5);
    inv4(&a)
}

#[no_mangle]
pub extern fn test() -> u16 {
    inv8(V8(0, 0, 0, 0, 1, 65520, 0, 2))
}
