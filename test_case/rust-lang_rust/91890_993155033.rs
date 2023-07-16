plain
     Running unittests (build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/alloc-e5ec6ba6e4af9b63)

running 270 tests
.................................................................................................... 100/270
.............................................................F..F................................... 200/270
failures:


---- raw_vec::tests::zst_grow_amortized_panic stdout ----
error: test failed, to rerun pass '-p alloc --lib'
thread 'raw_vec::tests::zst_grow_amortized_panic' panicked at 'assertion failed: `(left != right)`
 right: `0`', library/alloc/src/raw_vec.rs:364:9
note: panic did not contain expected string
note: panic did not contain expected string
      panic message: `"assertion failed: `(left != right)`\n  left: `0`,\n right: `0`"`,
 expected substring: `"divide by zero"`
---- raw_vec::tests::zst_grow_exact_panic stdout ----
thread 'raw_vec::tests::zst_grow_exact_panic' panicked at 'assertion failed: `(left != right)`
 right: `0`', library/alloc/src/raw_vec.rs:364:9
note: panic did not contain expected string
note: panic did not contain expected string
      panic message: `"assertion failed: `(left != right)`\n  left: `0`,\n right: `0`"`,
 expected substring: `"divide by zero"`
failures:
failures:
    raw_vec::tests::zst_grow_amortized_panic
    raw_vec::tests::zst_grow_exact_panic
test result: FAILED. 268 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:19:02
