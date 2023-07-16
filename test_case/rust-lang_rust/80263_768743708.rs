
---- sys::unix::process::process_inner::tests::test_command_fork_no_unwind stdout ----
ExitStatus(ExitStatus(139))
got=Ok(ExitStatus(ExitStatus(139)))
thread 'sys::unix::process::process_inner::tests::test_command_fork_no_unwind' panicked at 'assertion failed: signal == libc::SIGABRT || signal == libc::SIGILL', library/std/src/sys/unix/process/process_unix/tests.rs:21:5
