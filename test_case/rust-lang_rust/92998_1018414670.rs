plain
test time::tests::system_time_math ... ok

failures:

---- collections::hash::map::tests::test_try_reserve stdout ----
thread 'main' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: usize::MAX / 8 should trigger an OOM!', library/std/src/collections/hash/map/tests.rs:831:5

failures:
    collections::hash::map::tests::test_try_reserve


test result: FAILED. 499 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out

program exited (with status: 101), but EXIT_RUNTIME is not set, so halting execution but not exiting the runtime or preventing further async execution (build with EXIT_RUNTIME=1, if you want a true shutdown)
error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-emscripten" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:32:29
