rust
#[target_feature = "+sse"]
fn split_sse(f32x8) -> (f32x4, f32x4); // no op?
#[target_feature = "+avx"]
fn split_avx(f32x8) -> (f32x4, f32x4);
// ^^^^ copy the parts of a 256bit register into 2x128bit registers
