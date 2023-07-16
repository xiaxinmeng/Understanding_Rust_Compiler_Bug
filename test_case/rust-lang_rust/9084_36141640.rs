 rust
pub trait StrSlice<'a> {
...
    fn equals_ignore_case(&self, needle: &str) -> bool;
    fn starts_with_ignore_case(&self, needle: &str) -> bool;
    fn ends_with_ignore_case(&self, needle: &str) -> bool;
// or 
    fn equals_ignore_case(&self, needle: &str) -> bool;
...
// or
  fn compare(&self, needle: &str, ignore_case: bool) -> bool;
...
}
