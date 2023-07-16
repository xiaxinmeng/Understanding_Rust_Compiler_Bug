plain
test time::tests::system_time_math ... ok

failures:

---- sys::unix::process::process_inner::tests::test_command_fork_no_unwind stdout ----
thread 'main' panicked at 'failed to get command status: Os { code: 6, kind: WouldBlock, message: "Resource temporarily unavailable" }', library/std/src/sys/unix/process/process_unix/tests.rs:47:29
[library/std/src/sys/unix/process/process_unix/tests.rs:51] &got = Err(
    Any { .. },
)
thread 'main' panicked at 'panic unexpectedly propagated: Any { .. }', library/std/src/sys/unix/process/process_unix/tests.rs:52:22

failures:
    sys::unix::process::process_inner::tests::test_command_fork_no_unwind


test result: FAILED. 450 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out

program exited (with status: 101), but EXIT_RUNTIME is not set, so halting execution but not exiting the runtime or preventing further async execution (build with EXIT_RUNTIME=1, if you want a true shutdown)
error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-emscripten" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/alloc
Build completed unsuccessfully in 0:25:42
