rust
// Inefficiently compiled.
pub fn case_1(mut u: u32) -> u32 {
    if u > 0x7F {
        u = 0x7F;
    }
    // At this point u is 0..=0x7F and the mask operation does nothing.
    u &= 0x7F;
    u
}

pub fn case_2(mut u: u32) -> u32 {
    if u > 0x7F {
        u = 0x7F;
    }
    u
}
