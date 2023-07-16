plain
test result: ok. 318 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 3651 tests
..............................FFFi..iF.....iiiiii................................................... 100/3651
...........................................................ii....................................... 300/3651
.................................................................................................... 400/3651
..........i......................................................................................... 500/3651
..........................................................................................ii........ 600/3651
---
..............i.....................i.....................i......................................... 3600/3651
...................................................
failures:

---- src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::Simd<T,LANES>::deinterleave (line 325) stdout ----
error[E0432]: unresolved import `core_simd`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:327:5
  |
5 | use core_simd::Simd;
  |     ^^^^^^^^^ use of undeclared crate or module `core_simd`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::Simd<T,LANES>::interleave (line 263) stdout ----
error[E0432]: unresolved import `core_simd`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:265:5
  |
5 | use core_simd::Simd;
  |     ^^^^^^^^^ use of undeclared crate or module `core_simd`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 28) stdout ----
error[E0432]: unresolved imports `core_simd`, `Which::*`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:30:5
  |
5 | use core_simd::{Simd, simd_swizzle, Which};
  |     ^^^^^^^^^ use of undeclared crate or module `core_simd`
6 | use Which::*;


error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:36:9
   |
11 | let r = simd_swizzle!(a, b, [First(0), First(1), Second(2), Second(3)]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:40:9
   |
15 | let r = simd_swizzle!(a, b, [First(0), Second(0)]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 13) stdout ----
error[E0432]: unresolved import `core_simd`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:15:5
  |
5 | use core_simd::{Simd, simd_swizzle};
  |     ^^^^^^^^^ use of undeclared crate or module `core_simd`

error: cannot determine resolution for the macro `simd_swizzle`
 --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:19:9
  |
9 | let r = simd_swizzle!(v, [3, 0, 1, 2]);
  |
  |
  = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `simd_swizzle`
  --> src/../../portable-simd/crates/core_simd/src/swizzle.rs:23:9
   |
13 | let r = simd_swizzle!(v, [3, 1]);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::Simd<T,LANES>::deinterleave (line 325)
    src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::Simd<T,LANES>::interleave (line 263)
    src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 13)
    src/../../portable-simd/crates/core_simd/src/swizzle.rs - core_simd::swizzle::simd_swizzle (line 28)
test result: FAILED. 3615 passed; 4 failed; 32 ignored; 0 measured; 0 filtered out; finished in 60.09s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:03
