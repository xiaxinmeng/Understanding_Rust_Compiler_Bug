rust
/// Optimized as long as the compiler knows:
/// - (a) the value of y;
/// - and (b) that x <= u32::MAX - y.
/// Note that the order of (a) and (b) also matters.
pub fn optimized(x: u32, y: u32) -> u32 {
    assert!(y == 42); // cannot be moved down
    assert!(x <= u32::MAX - y);

    x.checked_add(y).unwrap()
}

/// The check for overflow is not yet optimized away, even though
/// x <= u32::MAX - y.
pub fn not_optimized_yet(x: u32, y: u32) -> u32 {
    assert!(x <= u32::MAX - y);

    x.checked_add(y).unwrap()
}
