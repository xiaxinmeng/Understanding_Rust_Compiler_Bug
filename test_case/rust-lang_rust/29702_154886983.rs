 rust
unsafe fn from_i8_unchecked(v: i8) -> Ordering {
    mem::transmute(v)
}
