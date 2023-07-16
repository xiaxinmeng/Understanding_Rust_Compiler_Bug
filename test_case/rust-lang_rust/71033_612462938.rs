rust
use std::mem;

fn main() {
    // Being valid utf8 is still a safety invariant of `str`.
    // As any method using `str` may depend on this invariant,
    // it would still be UB to use `str::from_utf8_unchecked`
    // or `str::as_bytes` here. 
    let s: &str = unsafe { std::mem::transmute(b"\xff\xff" as &[u8]) };
    let bytes: &[u8] = unsafe { std::mem::transmute(s) };
    assert_eq!(bytes, &[0xff, 0xff][..]);
}
