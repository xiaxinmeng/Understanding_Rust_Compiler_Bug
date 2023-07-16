plain
.................................................................................................... 500/599
...................................................................................................
failures:

---- src/str.rs - str::str::repeat (line 511) stdout ----
error: unused return value of `str::<impl str>::repeat` that must be used
  |
  |
5 | "0123456789abcdef".repeat(usize::MAX);
  |
note: the lint level is defined here
 --> src/str.rs:510:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:17:36
