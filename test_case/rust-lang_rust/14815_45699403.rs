 rust
#![crate_type = "lib"]

enum NodeContents<'a> {}

impl<'a> Drop for NodeContents<'a> {
  fn drop(&mut self) {}
}
