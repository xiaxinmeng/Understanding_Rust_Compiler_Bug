plain
running 92 tests
ii..i.....i......i.ii....i..........iii.................F.....i.............i...............
failures:

---- src/builtin.rs - builtin::UNFULFILLED_LINT_EXPECTATION (line 468) stdout ----
error[E0658]: the `#[expect]` attribute is an experimental feature
  |
  |
3 | #[expect(unused_variables)]
  |
  = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
  = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.

failures:
    src/builtin.rs - builtin::UNFULFILLED_LINT_EXPECTATION (line 468)
test result: FAILED. 78 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.63s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:22:31
