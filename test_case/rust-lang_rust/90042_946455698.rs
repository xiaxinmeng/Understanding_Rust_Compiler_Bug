plain
.................................F.................................................................. 1300/1340
........................................
failures:

---- slice::test_copy_within stdout ----
thread 'slice::test_copy_within' panicked at 'assertion failed: `(left == right)`
  left: `[72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 72, 101, 108]`,
 right: `[72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 72, 101, 108]`', library/core/tests/slice.rs:2051:5

failures:
    slice::test_copy_within


test result: FAILED. 1337 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.82s

error: test failed, to rerun pass '-p core --test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:21
