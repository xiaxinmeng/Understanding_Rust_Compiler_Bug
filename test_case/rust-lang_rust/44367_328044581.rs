rust
#[target_feature = "sse"]
fn foo(v: f32x8) -> f32x8 {
  // f32x8 has SSE ABI here
  let u = if std::host_feature(AVX) { 
      foo_avx(v)  // mismatched ABI: hard error (argument)
      // mismatched ABI: hard error (return type)
  } else {
      /* SSE code */
  }
  /* do something with u */
  u
}

#[target_feature = "avx"]
fn foo_avx(arg: f32x8) -> f32x8 { ... }
