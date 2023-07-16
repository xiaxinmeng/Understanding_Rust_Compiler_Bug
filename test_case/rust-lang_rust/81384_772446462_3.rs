rust
fn lt(&self, other: &Rhs) -> bool {
    matches!(self.partial_cmp(other), Some(Less))
}
