 rust
// Self = str
fn bytes<'a>(&'a self) -> Bytes<'a> {
    self.as_bytes().iter().map(|&b| b)
}
