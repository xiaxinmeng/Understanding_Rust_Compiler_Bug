rust
pub fn check(x: usize) -> usize {
    if x == usize::max_value() {
        abort();
    }
    x.checked_add(1).unwrap().wrapping_sub(1)
}
