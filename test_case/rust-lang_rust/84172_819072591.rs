plain
test sync::mpsc::sync_tests::destroy_upgraded_shared_port_when_sender_still_active ... ok
test sync::mpsc::sync_tests::oneshot_single_thread_try_recv_closed_with_data ... ok
test sync::mpsc::sync_tests::try_send1 ... ok
test sync::mpsc::sync_tests::try_recv_states ... ok
error: test failed, to rerun pass '-p std --lib'
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-e86754c3a5cb43e4` (signal: 11, SIGSEGV: invalid memory reference)


command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--frozen" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/obj/build/tmp/distcheck/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test --stage 2
Build completed unsuccessfully in 0:30:19
Build completed unsuccessfully in 0:30:19
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed

command did not execute successfully: "make" "check"
expected success, got: exit code: 2

