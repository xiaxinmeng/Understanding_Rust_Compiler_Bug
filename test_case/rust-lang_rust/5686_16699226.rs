 rust
#[abi = "rust-intrinsic"]
pub extern "rust-intrinsic" {
   #[fixed_stack_segment]
   pub fn sinf32(x: f32) -> f32;
}
