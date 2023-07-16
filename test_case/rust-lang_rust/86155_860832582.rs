rust
pub fn test_exported() {
  extern "C-unwind" {
    fn test();
  }
  test();
}
