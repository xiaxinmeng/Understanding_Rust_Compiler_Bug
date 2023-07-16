plain
running 16 tests
..............F.
failures:

---- tests::test_codegen_options_tracking_hash stdout ----
thread 'tests::test_codegen_options_tracking_hash' panicked at 'assertion failed: `(left != right)`
  left: `false`,
 right: `false`', compiler/rustc_interface/src/tests.rs:567:5


failures:
    tests::test_codegen_options_tracking_hash
    tests::test_codegen_options_tracking_hash

test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass '-p rustc_interface --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_interface" "--" "--quiet"


Build completed unsuccessfully in 0:21:55
