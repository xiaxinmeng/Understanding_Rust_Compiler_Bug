rust
if ty == unit {
    // fast path
} else {
    // slow path
    if let Ok(layout) = layout_of(ty) && layout.is_zst() {
        // slow path success
    } else {
        // slow path failure
    }
}
