plain

   Doc-tests rustc_lint

running 54 tests
.....F...Fii......F...................................

---- src/builtin.rs - builtin::ELLIPSIS_INCLUSIVE_RANGE_PATTERNS (line 1688) stdout ----
error[E0783]: `...` range patterns are deprecated
 --> src/builtin.rs:1691:5
---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0783`.
Couldn't compile the test.
---- src/array_into_iter.rs - array_into_iter::ARRAY_INTO_ITER (line 15) stdout ----
error[E0614]: type `{integer}` cannot be dereferenced
  |
  |
4 | [1, 2, 3].into_iter().for_each(|n| { *n; });

error: aborting due to previous error

For more information about this error, try `rustc --explain E0614`.
For more information about this error, try `rustc --explain E0614`.
Couldn't compile the test.
---- src/non_fmt_panic.rs - non_fmt_panic::NON_FMT_PANICS (line 20) stdout ----
error: 1 positional argument in format string, but no arguments were given
  |
3 | panic!("{}");
  |         ^^

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint" "--" "--quiet"


Build completed unsuccessfully in 0:24:12
