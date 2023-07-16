plain
.................................................................................................... 1300/1353
.....................................................
failures:

---- iter::adapters::take::test_take_advance_by stdout ----
thread 'iter::adapters::take::test_take_advance_by' panicked at 'attempt to subtract with overflow', /checkout/library/core/src/iter/adapters/take.rs:225:17

failures:
    iter::adapters::take::test_take_advance_by


test result: FAILED. 1350 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.28s

error: test failed, to rerun pass '-p core --test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:44
