rust
#[target_feature = "+sse"]
fn foo() {
  let a: fn(f32x8) -> f32x8;  // has type fn(f32x8["sse"]) -> f32x8["sse"]
}

#[target_feature = "+avx"]
fn bar() {
  let a: fn(f32x8) -> f32x8;  // has type fn(f32x8["avx"]) -> f32x8["avx"]
}

static a: fn(f32x8) -> f32x8;  // has type fn(f32x8["CRATE"]) -> f32x8["CRATE"]
// where CRATE is replaced with whatever feature the crate is compiled with
