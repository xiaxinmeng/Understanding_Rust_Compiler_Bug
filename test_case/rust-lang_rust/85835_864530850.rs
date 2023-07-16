plain
..............i.....................i.....................i.....................i................... 2900/2987
..i....................................................................................
failures:

---- src/iter/traits/collect.rs - iter::traits::collect::(ExtendA, ExtendB)::extend (line 373) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[0, 2, 4, 6]`,
 right: `[1, 3, 5, 7]`', src/iter/traits/collect.rs:7:1



failures:
failures:
    src/iter/traits/collect.rs - iter::traits::collect::(ExtendA, ExtendB)::extend (line 373)
test result: FAILED. 2959 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out; finished in 41.74s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:58
