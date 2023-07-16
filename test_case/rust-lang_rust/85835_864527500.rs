plain
..............i.....................i.....................i.....................i................... 2900/2987
..i....................................................................................
failures:

---- src/iter/traits/collect.rs - iter::traits::collect::(ExtendA, ExtendB)::extend (line 373) stdout ----
error[E0423]: expected value, found macro `vec`
   |
   |
10 | let mut nested_tuple = (vec![(1, -1)], vec[(2, -2)]);
   |
   |
help: use `!` to invoke the macro
   |
10 | let mut nested_tuple = (vec![(1, -1)], vec![(2, -2)]);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0423`.
For more information about this error, try `rustc --explain E0423`.
Couldn't compile the test.

failures:
    src/iter/traits/collect.rs - iter::traits::collect::(ExtendA, ExtendB)::extend (line 373)
test result: FAILED. 2959 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out; finished in 43.57s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:54
