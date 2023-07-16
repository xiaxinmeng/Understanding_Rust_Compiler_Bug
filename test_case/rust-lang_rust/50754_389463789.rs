rust
#![feature(lang_items)]
#![no_std]

#[lang = "panic_fmt"]
fn panic_fmt() {}

type V2 = (u16, u16);
type V4 = (u16, u16, u16, u16);
type V8 = (u16, u16, u16, u16, u16, u16, u16, u16);

fn xor2(lhs: V2, rhs: V2) -> V2 {
    let (a0, a1) = lhs;
    let (b0, b1) = rhs;
    (a0 ^ b0, a1 ^ b1)
}

fn mul2(x: V2, y: V2) -> V2 {
    let (b, a) = x;
    let (d, c) = y;
    let e = (a ^ b) & (c ^ d);
    ((b & d) ^ e, (a & c) ^ e)
}

fn split4(v: V4) -> (V2, V2) {
    let (x0, x1, x2, x3) = v;
    ((x0, x1), (x2, x3))
}

fn mul4(x: V4, y: V4) -> V4 {
    let (b, a) = split4(x);
    let (d, c) = split4(y);
    let e = xor2(a, b);
    let (x2, x3) = xor2(mul2(a, c), e);
    let (x0, x1) = xor2(mul2(b, d), e);
    (x0, x1, x2, x3)
}

fn inv4(v: V4) -> V4 {
    let a = (v.2, v.3);
    let b = (v.0, v.1);
    let d = mul2(a, b);
    let (x2, x3) = mul2(d, b);
    let (x0, x1) = mul2(d, a);
    (x0, x1, x2, x3)
}

fn split8(v: V8) -> (V4, V4) {
    let (x0, x1, x2, x3, x4, x5, x6, x7) = v;
    ((x0, x1, x2, x3), (x4, x5, x6, x7))
}

fn inv8(v: V8) -> V8 {
    let (b, a) = split8(v);
    let (x0, x1, x2, x3) = mul4(b, a);
    let (x4, x5, x6, x7) = mul4(inv4(a), b);
    (0, !x1, x2, x3, x4, !x5, !x6, x7)
}

#[no_mangle]
pub fn test() -> u16 {
    let tmp0 = (1, 1, 1, 1, 1, 1, 1, 1);
    inv8(tmp0); // command out to get correct value.

    let tmp4 = (65525, 65530, 15, 0, 15, 65520, 5, 10);
    let tmp5 = inv8(tmp4);
    tmp5.5
}
