 rust
fn add(self, other: Duration) -> Duration {
    self.checked_add(other).expect("overflow when adding durations")
}
