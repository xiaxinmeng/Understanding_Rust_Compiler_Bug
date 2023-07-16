plain
---- [ui] tests/ui/abi/stack-probes.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/a"
--- stdout -------------------------------
status: signal: 6 (SIGABRT) (core dumped)
stderr: 
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow


status: signal: 11 (SIGSEGV) (core dumped)
stderr:
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: stderr.contains(\"has overflowed its stack\\n\")', fake-test-src-base/abi/stack-probes.rs:106:5
------------------------------------------


---- [ui] tests/ui/abi/stack-probes-lto.rs stdout ----
---- [ui] tests/ui/abi/stack-probes-lto.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a"
--- stdout -------------------------------
status: signal: 6 (SIGABRT) (core dumped)
stderr: 
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow


status: signal: 11 (SIGSEGV) (core dumped)
stderr:
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: stderr.contains(\"has overflowed its stack\\n\")', fake-test-src-base/abi/stack-probes.rs:106:5
------------------------------------------



