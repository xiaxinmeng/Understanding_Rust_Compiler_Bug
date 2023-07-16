plain
  |
7 | fn make_string(a: u32, b: &str) -> String {
  |    -----------                     ^^^^^^ expected struct `String`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     format!("{b} {a}");
  |                       - help: consider removing this semicolon
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:15:47
