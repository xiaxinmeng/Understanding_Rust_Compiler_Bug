plain
...........i........................................................................................ 3600/3603
...
failures:

---- src/slice/raw.rs - slice::raw::from_mut_ptr_range (line 242) stdout ----
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
  |
8 | let x = [1, 2, 3];
  |     - help: consider changing this to be mutable: `mut x`
9 | let range = x.as_mut_ptr_range();
  |             ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:20:28
