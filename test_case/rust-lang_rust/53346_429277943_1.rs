
error[E1337]: `__m256` on C FFI requires `#[target_feature(enable = "avx")]`
 --> src/main.rs:7:15
  |
7 |     fn foo(x: __m25a6) -> __m256;
  |               ^^^^^^^
