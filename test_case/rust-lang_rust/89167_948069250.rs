plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0439]: invalid `simd_shuffle`, needs length: `simd_shuffle`
  --> library/core/src/../../portable-simd/crates/core_simd/src/intrinsics.rs:57:5
   |
57 |     pub(crate) fn simd_shuffle<T, U, V>(x: T, y: T, idx: U) -> V;

For more information about this error, try `rustc --explain E0439`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:09
