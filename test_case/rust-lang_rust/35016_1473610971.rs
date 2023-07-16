rust
// In, say, mod.rs
#[path="."]
mod foo {
  #[path="."]
  mod bar {
    mod baz;
  }
}
