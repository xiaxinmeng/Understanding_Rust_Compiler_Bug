 rust
#![feature(collections_drain)]
#![crate_type="lib"]

#[inline(never)]
pub fn remove_range(v: &mut Vec<u8>, a: usize, b: usize) {
    if a <= b && b <= v.len() {
        v.drain(a..b);
    }
}
