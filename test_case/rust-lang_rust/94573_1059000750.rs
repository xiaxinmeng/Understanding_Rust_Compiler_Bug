rust
fn has_more_than_two_dashes(s: &[u8]) -> bool {
  sl.iter().filter(|&&c| c == b'-').nth(1).is_some()
}
