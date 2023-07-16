plain
test time::tests::instant_monotonic_concurrent ... ok

failures:

---- sys::unix::process::process_inner::tests::test_command_fork_no_unwind stdout ----
[library/std/src/sys/unix/process/process_unix/tests.rs:49] st = ExitStatus(
    unix_wait_status(
    ),
)
)
[library/std/src/sys/unix/process/process_unix/tests.rs:52] &got = Ok(
    ExitStatus(
        unix_wait_status(
        ),
    ),
)
)
[library/std/src/sys/unix/process/process_unix/tests.rs:54] status = ExitStatus(
    unix_wait_status(
    ),
)
)
thread 'sys::unix::process::process_inner::tests::test_command_fork_no_unwind' panicked at 'assertion failed: signal == libc::SIGABRT || signal == libc::SIGILL || signal == libc::SIGTRAP', library/std/src/sys/unix/process/process_unix/tests.rs:56:5

failures:
    sys::unix::process::process_inner::tests::test_command_fork_no_unwind


test result: FAILED. 925 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 439.88s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:47:24
