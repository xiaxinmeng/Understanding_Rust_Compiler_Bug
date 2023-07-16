rust
#[repr(simd)] f64x4(f64, f64, f64, f64);
#[target_feature(enable = "avx")] fn foo(x: f64x4) { ... }
#[target_feature(enable = "sse")] fn bar(x: f64x4} { foo(x) }
