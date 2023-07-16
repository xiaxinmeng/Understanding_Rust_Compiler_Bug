plain
  left: `1029918945431836347`,
 right: `8612613394021905856`', compiler/rustc_interface/src/tests.rs:85:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tests::test_debugging_options_tracking_hash stdout ----
thread 'tests::test_debugging_options_tracking_hash' panicked at 'assertion failed: `(left != right)`
 right: `10200085699698919768`', compiler/rustc_interface/src/tests.rs:92:5


failures:
failures:
    tests::test_debugging_options_tracking_hash
    tests::test_output_types_tracking_hash_different_paths

test result: FAILED. 13 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass '-p rustc_interface --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_interface" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:22:41
