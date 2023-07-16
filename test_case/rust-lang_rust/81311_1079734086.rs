rust
// inner crate
pub type DMatrix = Vec<u32>;

// outer crate
pub use inner::DMatrix;
