plain
---- [ui] src/test/ui/process/process-panic-after-fork.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a"
--- stdout -------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a", waiting for result
--- stderr -------------------------------
--- stderr -------------------------------
engaging allocator trap, parent pid=9715
panicked after panic::always_abort(), aborting.
panicked after panic::always_abort(), aborting.
[/checkout/src/test/ui/process/process-panic-after-fork.rs:78] status = ExitStatus(
    unix_wait_status(
    ),
)
)
thread 'main' panicked at 'assertion failed: signal == libc::SIGABRT || signal == libc::SIGILL || signal == libc::SIGTRAP', /checkout/src/test/ui/process/process-panic-after-fork.rs:80:5
------------------------------------------



