plain
.........i.......................................................................................... 3600/3601
.
failures:

---- src/char/methods.rs - char::methods::char::from_digit (line 231) stdout ----
error: unused return value of `from_digit` that must be used
  |
  |
7 | char::from_digit(1, 37);
  |
note: the lint level is defined here
 --> src/char/methods.rs:229:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:18:28
