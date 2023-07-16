plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0432]: unresolved imports `crate::LaneCount`, `crate::Simd`, `crate::SimdElement`, `crate::SupportedLaneCount`
 --> library/core/src/../../portable-simd/crates/core_simd/src/swizzle.rs:2:13
  |
2 | use crate::{LaneCount, Simd, SimdElement, SupportedLaneCount};
  |             ^^^^^^^^^  ^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^ no `SupportedLaneCount` in the root
  |             |          |     |
  |             |          |     no `SimdElement` in the root
  |             |          no `Simd` in the root
  |             |          help: a similar name exists in the module (notice the capitalization): `simd`
  |             no `LaneCount` in the root
For more information about this error, try `rustc --explain E0432`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
