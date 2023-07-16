rust
fn as_nanos(&self) -> u128 {
    self.secs as u128 * 1_000_000_000 + self.nanos as u128
}
