plain
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- sys::unix::process::process_inner::tests::test_command_fork_no_unwind stdout ----
ExitStatus(ExitStatus(139))
got=Ok(ExitStatus(ExitStatus(139)))
thread 'sys::unix::process::process_inner::tests::test_command_fork_no_unwind' panicked at 'assertion failed: signal == libc::SIGABRT || signal == libc::SIGILL', library/std/src/sys/unix/process/process_unix/tests.rs:21:5

failures:
    sys::unix::process::process_inner::tests::test_command_fork_no_unwind


test result: FAILED. 844 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.31s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i686-unknown-linux-musl" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
Build completed unsuccessfully in 0:26:14
