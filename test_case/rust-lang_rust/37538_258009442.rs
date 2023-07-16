 rust
/// Left shift that does not panic when shifting by the integer width.
fn shift_left(x: u8, shift: usize) -> u8 {
    debug_assert!(shift <= 8);

    // Rust panics when shifting by the integer width, so we have to treat
    // that case separately.
    if shift >= 8 { 0 } else { x << shift }
}
