plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0432]: unresolved import `core::simd`
 --> library/core/tests/simd.rs:1:11
  |
1 | use core::simd::f32x4;
  |           ^^^^ could not find `simd` in `core`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
