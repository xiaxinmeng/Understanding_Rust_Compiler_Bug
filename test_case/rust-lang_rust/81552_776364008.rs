rust
// This is passed in function calls as <i1 x 4> in LLVM IR.
#[repr(llvm_simd_bitmask(4))]
struct i1x4(u8);
