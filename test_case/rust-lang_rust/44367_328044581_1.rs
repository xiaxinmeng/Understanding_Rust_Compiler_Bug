rust
#[target_feature = "sse"]
fn foo(v: f32x8) -> f32x8 {
  // f32x8 has SSE ABI here
  let u = if std::host_feature(AVX) { 
      // foo_avx(v) // ERROR: mismatched ABIs (2x arg and ret type)
      // foo_avx(v as f32x8) // ERROR: mismatched ABIs (1x ret type)
      foo_avx(v as f32x8) as f32x8 // OK
  } else {
      /* SSE code */
  }
  /* do something with u */
  u
}
