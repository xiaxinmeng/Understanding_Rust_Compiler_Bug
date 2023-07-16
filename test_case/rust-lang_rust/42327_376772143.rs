rust
fn cmp(&self, other: &self) -> Ordering try {
    self.a.cmp(&other.a)?;
    self.b.cmp(&other.b)?;
}
