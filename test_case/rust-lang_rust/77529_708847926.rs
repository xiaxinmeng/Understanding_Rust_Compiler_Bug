rust
   Compiling approx v0.3.2
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> src/simd_llvm.rs:13:8
   |
13 | extern "platform-intrinsic" {
   |        ^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information

warning: unused import: `crate::simd_llvm`
  --> src/vec.rs:21:5
   |
21 | use crate::simd_llvm;
   |     ^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0658`.
