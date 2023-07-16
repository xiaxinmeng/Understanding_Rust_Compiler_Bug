plain
failures:

---- [ui] src/test/ui/codegen/issue-28950.rs stdout ----

error: test run failed!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen/issue-28950/a"
stdout: none
--- stderr -------------------------------
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] src/test/ui/lto/thin-lto-inlines.rs stdout ----
