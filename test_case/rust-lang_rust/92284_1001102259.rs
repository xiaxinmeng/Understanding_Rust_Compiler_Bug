plain
.................................................................................................... 1400/1402
..
failures:

---- iter::adapters::skip::test_skip_advance_by stdout ----
thread 'iter::adapters::skip::test_skip_advance_by' panicked at 'attempt to subtract with overflow', /checkout/library/core/src/iter/adapters/skip.rs:125:30

failures:
    iter::adapters::skip::test_skip_advance_by


test result: FAILED. 1399 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.52s

error: test failed, to rerun pass '-p core --test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:44
