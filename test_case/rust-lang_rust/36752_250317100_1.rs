 rust
fn testroutine(start: u8, end: u8) -> u8 {
    (start..end+1).map(|i| i * i).sum()
}
