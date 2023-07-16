rust
pub fn midpoint_simple(x: u64, y: u64) -> u64 {
    ((x as u128 + y as u128) / 2) as u64
}
