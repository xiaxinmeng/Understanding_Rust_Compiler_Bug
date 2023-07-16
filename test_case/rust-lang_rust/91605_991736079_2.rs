rust
// examples/ex.rs
fn main() {
  cfg_if::cfg_if! { if #[cfg(unix)] {} else {} }
  doc_test::heyo();
}
