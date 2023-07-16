plain

   Doc-tests core

running 3651 tests
................................FiF.i......iiiiii................................................... 100/3651
...........................................................ii....................................... 300/3651
.................................................................................................... 400/3651
..........i......................................................................................... 500/3651
..........................................................................................ii........ 600/3651
---
..............i.....................i.....................i......................................... 3600/3651
...................................................
failures:

---- src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 13) stdout ----
error[E0432]: unresolved import `core::simd::simd_swizzle`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:16:53
  |
6 | #[cfg(not(feature = "std"))] use core::simd::{Simd, simd_swizzle};
  |                                                     ^^^^^^^^^^^^ no `simd_swizzle` in `simd`
  |
  = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
  |
6 - #[cfg(not(feature = "std"))] use core::simd::{Simd, simd_swizzle};
6 + #[cfg(not(feature = "std"))] use core::{simd_swizzle, simd::{Simd}};


error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:24:9
   |
14 | let r = simd_swizzle!(v, [3, 1]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:20:9
   |
10 | let r = simd_swizzle!(v, [3, 0, 1, 2]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 29) stdout ----
error[E0432]: unresolved import `core::simd::simd_swizzle`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:32:53
  |
6 | #[cfg(not(feature = "std"))] use core::simd::{Simd, simd_swizzle, Which};
  |                                                     ^^^^^^^^^^^^ no `simd_swizzle` in `simd`
  |
  = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
  |
6 - #[cfg(not(feature = "std"))] use core::simd::{Simd, simd_swizzle, Which};
6 + #[cfg(not(feature = "std"))] use core::{simd_swizzle, simd::{Simd, Which}};


error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:42:9
   |
16 | let r = simd_swizzle!(a, b, [First(0), Second(0)]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:38:9
   |
12 | let r = simd_swizzle!(a, b, [First(0), First(1), Second(2), Second(3)]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: unused import: `Which::*`
error: unused import: `Which::*`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:33:5
7 | use Which::*;
  |     ^^^^^^^^
  |
note: the lint level is defined here
note: the lint level is defined here
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:27:9
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`


error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.

failures:
    src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 13)
    src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 29)
test result: FAILED. 3617 passed; 2 failed; 32 ignored; 0 measured; 0 filtered out; finished in 60.11s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:21
