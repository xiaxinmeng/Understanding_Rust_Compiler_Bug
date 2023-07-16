plain
---- src/result.rs - result::Result<T, E>::check (line 867) stdout ----
error[E0658]: use of unstable library feature 'result_check'
  --> src/result.rs:875:15
   |
11 | assert!(value.check(check_non_zero).is_err());
   |
   = help: add `#![feature(result_check)]` to the crate attributes to enable

error: aborting due to previous error
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:18:50
