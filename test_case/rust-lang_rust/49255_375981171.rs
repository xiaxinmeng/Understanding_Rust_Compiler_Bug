
[01:18:41] failures:
[01:18:41] 
[01:18:41] ---- [run-pass] run-pass/needless_lifetimes_impl_trait.rs stdout ----
[01:18:41]      
[01:18:41] error: compilation failed!
[01:18:41] status: exit code: 101
[01:18:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/run-pass/needless_lifetimes_impl_trait.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/re
lease/build/clippy-49a9977e3d89da30/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-49a9977e3d89da
30/out/test_build_base/needless_lifetimes_impl_trait.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-L" "/check
out/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-49a9977e3d89da30/out/test_build_base/needless_lifetimes_impl_trait.stage-id.aux"
[01:18:41] stdout:
[01:18:41] ------------------------------------------
[01:18:41] 
[01:18:41] ------------------------------------------
[01:18:41] stderr:
[01:18:41] ------------------------------------------
[01:18:41] error: this feature has been stable since 1.26.0. Attribute no longer needed
[01:18:41]  --> tests/run-pass/needless_lifetimes_impl_trait.rs:3:12
[01:18:41]   |
[01:18:41] 3 | #![feature(conservative_impl_trait)]
[01:18:41]   |            ^^^^^^^^^^^^^^^^^^^^^^^
[01:18:41]   |
[01:18:41]   = note: `-D stable-features` implied by `-D warnings`
[01:18:41] 
[01:18:41] error: aborting due to previous error
[01:18:41] 
[01:18:41] 
[01:18:41] ------------------------------------------
[01:18:41] 
[01:18:41] thread '[run-pass] run-pass/needless_lifetimes_impl_trait.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.8/src/runtest.rs:2539:9
[01:18:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:18:41] 
[01:18:41] 
[01:18:41] failures:
[01:18:41]     [run-pass] run-pass/needless_lifetimes_impl_trait.rs
[01:18:41] 
[01:18:41] test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:41] 
[01:18:41] test compile_test ... FAILED
[01:18:41] 
[01:18:41] failures:
[01:18:41] 
[01:18:41] ---- compile_test stdout ----
[01:18:41]      thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.8/src/lib.rs:89:22
[01:18:41] 
[01:18:41] 
[01:18:41] failures:
[01:18:41]     compile_test
[01:18:41] 
[01:18:41] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:41] 
[01:18:41] error: test failed, to rerun pass '--test compile-test'
[01:18:41] 
[01:18:41] 
[01:18:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml"
