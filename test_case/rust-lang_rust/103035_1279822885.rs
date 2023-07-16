plain
---- [ui] src/test/ui/process/process-panic-after-fork.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a"
stdout: none
--- stderr -------------------------------
engaging allocator trap, parent pid=81651
panicked after panic::always_abort(), aborting.
panicked after panic::always_abort(), aborting.
[/checkout/src/test/ui/process/process-panic-after-fork.rs:79] status = ExitStatus(
    unix_wait_status(
    ),
)
panicked at /checkout/src/test/ui/process/process-panic-after-fork.rs:150:13
panicked after panic::always_abort(), aborting.
panicked after panic::always_abort(), aborting.
[/checkout/src/test/ui/process/process-panic-after-fork.rs:79] status = ExitStatus(
    unix_wait_status(
    ),
)
)
[/checkout/src/test/ui/process/process-panic-after-fork.rs:79] status = ExitStatus(
    unix_wait_status(
    ),
)
)
thread 'main' panicked at 'assertion failed: signal == libc::SIGABRT || signal == libc::SIGILL || signal == libc::SIGTRAP', /checkout/src/test/ui/process/process-panic-after-fork.rs:83:5
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

