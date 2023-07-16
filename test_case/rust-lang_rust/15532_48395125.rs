 rust
pub fn with(mut self, f: |&mut Vec<T>|) -> Vec<T> {
    f(&mut self);
    self
}
