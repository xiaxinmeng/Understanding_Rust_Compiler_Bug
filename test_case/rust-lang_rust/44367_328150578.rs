rust
#[target_feature = "+sse"]
fn merge_sse(x: (f32x4, f32x4)) -> f32x8;  // no op?
#[target_feature = "+avx"]
fn merge_avx(x: (f32x4, f32x4)) -> f32x8; 
// ^^^^ copy 2x128bit registers to 1x256register
