plain

failures:

---- thread::local::tests::join_orders_after_tls_destructors stdout ----
thread 'thread::local::tests::join_orders_after_tls_destructors' panicked at 'internal error: entered unreachable code: sync state: Err(2)', library/std/src/thread/local/tests.rs:312:22

failures:
    thread::local::tests::join_orders_after_tls_destructors


test result: FAILED. 859 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.35s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "14" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:26:39
