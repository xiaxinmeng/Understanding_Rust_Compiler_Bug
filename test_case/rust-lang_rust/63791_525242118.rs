rust
pub fn deconstruct(x: [u16; 8], bit: u32) -> u32 {
    fn pb(x: u16, bit: u32, shift: u32) -> u32 {
        // can't reproduce with different order (e.g. mask then shift)
        u32::from((x >> bit) & 1) << shift 
    }
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
