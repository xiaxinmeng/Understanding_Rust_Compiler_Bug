plain
test thread::tests::sleep_ms_smoke ... ok
test thread::tests::test_named_thread ... ok
test sync::mpsc::sync_tests::stress_shared ... ok
test thread::tests::test_join_panic ... ok
thread '<unnamed>' panicked at 'use of std::thread::current() is not possible after the thread's local data has been destroyed', library/std/src/thread/mod.rs:651:35
test thread::tests::test_simple_newsched_spawn ... ok
test thread::tests::test_simple_newsched_spawn ... ok
fatal runtime error: failed to initiate panic, error 5
error: test failed, to rerun pass '-p std --lib'
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/deps/std-70c00059e8a27a8e` (signal: 6, SIGABRT: process abort signal)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i686-unknown-linux-musl" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
Build completed unsuccessfully in 0:28:12
