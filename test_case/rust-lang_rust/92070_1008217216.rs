plain
.................i....................i.....................i.....................i................. 3600/3697
....i............................................................................................
failures:

---- src/iter/adapters/mod.rs - iter::adapters::SourceIter (line 81) stdout ----
error[E0599]: the method `as_inner` exists for struct `Map<std::array::IntoIter<{integer}, 3_usize>, [closure@src/iter/adapters/mod.rs:7:42: 7:51]>`, but its trait bounds were not satisfied
   |
   |
9  | let mut remainder = std::mem::replace(unsafe { iter.as_inner() }, Vec::new().into_iter());
   |                                                     ^^^^^^^^ method cannot be called on `Map<std::array::IntoIter<{integer}, 3_usize>, [closure@src/iter/adapters/mod.rs:7:42: 7:51]>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/array/iter.rs:14:1
   |
14 | pub struct IntoIter<T, const N: usize> {
   | -------------------------------------- doesn't satisfy `_: SourceIter`
   | -------------------------------------- doesn't satisfy `_: SourceIter`
   |
  ::: /checkout/library/core/src/iter/adapters/map.rs:61:1
   |
61 | pub struct Map<I, F> {
   | -------------------- doesn't satisfy `_: SourceIter`
   |
   = note: the following trait bounds were not satisfied:
           `std::array::IntoIter<{integer}, 3_usize>: SourceIter`
           which is required by `Map<std::array::IntoIter<{integer}, 3_usize>, [closure@src/iter/adapters/mod.rs:7:42: 7:51]>: SourceIter`
error: unused import: `std::iter::SourceIter`
 --> src/iter/adapters/mod.rs:83:5
  |
5 | use std::iter::SourceIter;
---
---- src/result.rs - result (line 478) stdout ----
error[E0308]: mismatched types
 --> src/result.rs:482:33
  |
7 | let v: Vec<Result<i32, &str>> = [Ok(1), Ok(2), Ok(21)];
  |        ----------------------   ^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_vec()`
  |        |                        |
  |        |                        expected struct `Vec`, found array of 3 elements
  |
  |
  = note: expected struct `Vec<Result<i32, &str>>`
              found array `[Result<{integer}, _>; 3]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:02
