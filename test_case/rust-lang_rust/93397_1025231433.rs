plain
........i.....................i..................................................................... 3700/3723
.......................
failures:

---- src/slice/mod.rs - slice::[f32]::sort (line 4001) stdout ----
error[E0034]: multiple applicable items in scope
  |
7 | v.sort();
7 | v.sort();
  |   ^^^^ multiple `sort` found
  |
  = note: candidate #1 is defined in an impl for the type `[T]`
  = note: candidate #2 is defined in an impl for the type `[f32]`
  = note: candidate #3 is defined in an impl for the type `[f64]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[f64]::sort (line 4032) stdout ----
error[E0034]: multiple applicable items in scope
  |
7 | v.sort();
7 | v.sort();
  |   ^^^^ multiple `sort` found
  |
  = note: candidate #1 is defined in an impl for the type `[T]`
  = note: candidate #2 is defined in an impl for the type `[f32]`
  = note: candidate #3 is defined in an impl for the type `[f64]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:17:43
