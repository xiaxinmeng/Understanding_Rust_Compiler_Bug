
#![crate_type = "dylib"]
pub mod access
{
  pub use super::*; // <--- removing the public visibility makes the compiler not stack overflow.
}
