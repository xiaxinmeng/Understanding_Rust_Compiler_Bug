rust
pub const fn midpoint(a: u64, b: u64) -> u64 {
    let b1 = b + ((b < a) as u64);
    return ((a ^ b1) >> 1) + (a & b1);
}
