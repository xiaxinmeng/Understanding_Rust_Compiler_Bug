plain
running 57 tests
.........ii................F.............................
failures:

---- src/pass_by_value.rs - pass_by_value::RUSTC_PASS_BY_VALUE (line 16) stdout ----
error[E0658]: #[rustc_pass_by_value] is used to mark types that must be passed by value instead of reference.
  |
3 | #[rustc_pass_by_value]
  | ^^^^^^^^^^^^^^^^^^^^^^
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint" "--" "--quiet"


Build completed unsuccessfully in 0:25:07
