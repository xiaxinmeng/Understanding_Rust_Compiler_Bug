rust
fn main() {
    let x = [0, 0, 0, 0, 0, 0, 0, 1];
    let x = unslice(x).0;
    assert_eq!(x, 1 << 31);
}

#[inline(never)] // can't reproduce without inline(never)
fn unslice(x: [u16; 8]) -> (u32, u32, u32) {
    let a = deconstruct(x, 0);
    let b = deconstruct(x, 1); // needs two calls with different parameter
    (a, b, 0) // can't reproduce without the third value in the tuple
}

#[inline(never)] // can't reproduce without inline(never)
fn deconstruct(x: [u16; 8], bit: u32) -> u32 {
    // this part needs to get vectorized
    pb(x[0], bit + 1, 16) | pb(x[1], bit + 1, 17)
        | pb(x[2], bit + 1, 18) | pb(x[3], bit + 1, 19)
        | pb(x[4], bit + 1, 20) | pb(x[5], bit + 1, 21)
        | pb(x[6], bit + 1, 22) | pb(x[7], bit + 1, 23)
        | pb(x[0], bit, 24) | pb(x[1], bit, 25)
        | pb(x[2], bit, 26) | pb(x[3], bit, 27)
        | pb(x[4], bit, 28) | pb(x[5], bit, 29)
        | pb(x[6], bit, 30) | pb(x[7], bit, 31)
}

fn pb(x: u16, bit: u32, shift: u32) -> u32 {
    u32::from((x >> bit) & 1) << shift // can't reproduce with different order (e.g. mask then shift)
}
