plain
.................................................................................................... 600/658
..........................................................
failures:

---- str::slice_index::test_slice_fail_truncated_2 stdout ----
thread 'str::slice_index::test_slice_fail_truncated_2' panicked at 'byte index 1024 is out of bounds of `Lor`[...]', library/alloc/tests/str.rs:723:18
note: panic did not contain expected string
      panic message: `"byte index 1024 is out of bounds of `Lor`[...]"`,
 expected substring: `"luctus, im`[...]"`
---- str::slice_index::test_slice_fail_truncated_1 stdout ----
thread 'str::slice_index::test_slice_fail_truncated_1' panicked at 'byte index 1024 is out of bounds of `Lor`[...]', library/alloc/tests/str.rs:717:18
note: panic did not contain expected string
      panic message: `"byte index 1024 is out of bounds of `Lor`[...]"`,
 expected substring: `"byte index 1024 is out of bounds of `Lorem ipsum dolor sit amet"`
failures:
    str::slice_index::test_slice_fail_truncated_1
    str::slice_index::test_slice_fail_truncated_2
error: test failed, to rerun pass '-p alloc --test collectionstests'
error: test failed, to rerun pass '-p alloc --test collectionstests'

test result: FAILED. 656 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:09
