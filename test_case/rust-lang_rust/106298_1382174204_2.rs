
failures:

---- [ui] rust.git/tests/ui/proc-macro/issue-106298-double-panic-in-proc-macro.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: <snip>
stdout: none
--- stderr -------------------------------
thread panicked while panicking. aborting.
------------------------------------------



failures:
    [ui] rust.git/tests/ui/proc-macro/issue-106298-double-panic-in-proc-macro.rs
