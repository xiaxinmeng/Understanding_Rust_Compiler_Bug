
---- compile_test stdout ----
thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    compile_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test compile-test'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
expected success, got: exit code: 101


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/clippy
