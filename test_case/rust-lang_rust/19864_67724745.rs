 rust
pub struct IoError {
  pub kind: IoErrorKind // 2 words because a single variant has a uint associated with it
  pub desc: &'static str // 2 words - the size of a string slice
  pub detail: Option<String> // 4 words - One for the discriminant 3 for the String.
}
