plain
.................................................................................................... 600/665
.................................................................
failures:

---- vec::test_dedup_by stdout ----
thread 'vec::test_dedup_by' panicked at 'assertion failed: `(left == right)`
  left: `[("foo", 1), ("bar", 12)]`,
 right: `[("foo", 3), ("bar", 12)]`', library/alloc/tests/vec.rs:402:5

failures:
    vec::test_dedup_by


test result: FAILED. 664 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s

error: test failed, to rerun pass '-p alloc --test collectionstests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:18:42
