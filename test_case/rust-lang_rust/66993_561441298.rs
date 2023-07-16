rust
pub unsafe fn rem(value: i8) -> i8 {
    match value.rem_euclid(7)
    {
        remainder @ 0..=6 => remainder,
        _ => std::hint::unreachable_unchecked(),
    }
}
