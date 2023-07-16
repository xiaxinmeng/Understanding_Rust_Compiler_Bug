 rust
fn bar(x: u32, y: u32) -> u32 {
    x.checked_add(&y).unwrap()
}
