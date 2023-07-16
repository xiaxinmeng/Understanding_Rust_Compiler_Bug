rust
fn peculiar() -> impl Fn(u8) -> u8 {
    return |x| x + 1;
}
