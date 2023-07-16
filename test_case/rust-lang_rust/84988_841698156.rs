rust
// Use your favorite wasm target feature instead.
#[target_feature(enable = "sse4")] unsafe fn foo_sse4() {}

fn main() {
  unsafe { foo_sse4() }
}
