plain
.........i.......................................................................................... 3600/3601
.
failures:

---- src/char/methods.rs - char::methods::char::to_digit (line 329) stdout ----
error: unused return value of `char::methods::<impl char>::to_digit` that must be used
  |
  |
5 | '1'.to_digit(37);
  |
note: the lint level is defined here
 --> src/char/methods.rs:327:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:20:49
