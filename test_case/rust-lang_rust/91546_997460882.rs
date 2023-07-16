plain
     Running tests/lib.rs (build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-6f16310608e561c6)

running 679 tests
.................................................................................................... 100/679
...................................................................................F...F.F......F... 200/679
.................................................................................................... 400/679
.................................................................................................... 500/679
.................................................................................................... 600/679
...............................................................................
...............................................................................
failures:

---- slice::test_rsplitator_inclusive stdout ----
thread 'slice::test_rsplitator_inclusive' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[[]]`', library/alloc/tests/slice.rs:1027:5
---- slice::test_rsplitator_inclusive_reverse stdout ----
thread 'slice::test_rsplitator_inclusive_reverse' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
  left: `[]`,
 right: `[[]]`', library/alloc/tests/slice.rs:1047:5
---- slice::test_rsplitator_mut_inclusive stdout ----
---- slice::test_rsplitator_mut_inclusive stdout ----
thread 'slice::test_rsplitator_mut_inclusive' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[[]]`', library/alloc/tests/slice.rs:1067:5
---- slice::test_rsplitator_mut_inclusive_reverse stdout ----
thread 'slice::test_rsplitator_mut_inclusive_reverse' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
  left: `[]`,
 right: `[[]]`', library/alloc/tests/slice.rs:1087:5

failures:
    slice::test_rsplitator_inclusive
    slice::test_rsplitator_inclusive_reverse
---

error: test failed, to rerun pass '-p alloc --test collectionstests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:20:12
