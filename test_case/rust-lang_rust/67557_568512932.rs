rust
// compile-flags:-Zmir-opt-level=3

#[inline(never)]
fn assert_1234(x: &[u8]) {
    assert_eq!(x, &[1, 2, 3, 4]);
}

#[inline(always)]
fn ret_slice() -> &'static [u8] {
    &[1, 2, 3, 4]
}

fn main() {
    let promoted: &'static [u8] = &[0, 0, 0, 0];
    let slice: &'static [u8] = ret_slice();
    assert_1234(slice);
}
