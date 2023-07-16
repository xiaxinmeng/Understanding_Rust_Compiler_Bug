plain
failures:

---- [ui] src/test/ui/codegen/issue-28950.rs stdout ----

error: test run failed!
status: signal: 6 (SIGABRT) (core dumped)
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen/issue-28950" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen/issue-28950/a"
stdout: none
--- stderr -------------------------------
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow



failures:
