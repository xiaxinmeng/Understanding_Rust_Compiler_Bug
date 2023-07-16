
pub fn eq_ignore_ascii_case<C: Borrow<char>>(&self, other: C) -> bool {
    self.to_ascii_lowercase() == other.borrow().to_ascii_lowercase()
}
