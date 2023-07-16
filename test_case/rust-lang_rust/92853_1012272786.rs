plain
................i.....................i.....................i.....................i................. 3600/3697
....i............................................................................................
failures:

---- src/slice/index.rs - slice::index::range (line 496) stdout ----
error: unused return value of `range` that must be used
  |
  |
8 | slice::range(2..1, ..3);
  |
note: the lint level is defined here
 --> src/slice/index.rs:494:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_must_use)]` implied by `#[deny(warnings)]`

error: aborting due to previous error

Couldn't compile the test.
---- src/slice/index.rs - slice::index::range (line 512) stdout ----
error: unused return value of `range` that must be used
  |
  |
8 | slice::range(1..=usize::MAX, ..3);
  |
note: the lint level is defined here
 --> src/slice/index.rs:510:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_must_use)]` implied by `#[deny(warnings)]`

error: aborting due to previous error

Couldn't compile the test.
---- src/slice/index.rs - slice::index::range (line 504) stdout ----
error: unused return value of `range` that must be used
  |
  |
8 | slice::range(1..4, ..3);
  |
note: the lint level is defined here
 --> src/slice/index.rs:502:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:22:15
