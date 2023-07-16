
---- [ui] ui/issues/issue-24313.rs stdout ----

error: test run failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24313/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'bad failure message:
thread panicked while processing panic. aborting.

', /checkout/src/test/ui/issues/issue-24313.rs:23:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

------------------------------------------



failures:
    [ui] ui/issues/issue-24313.rs

test result: FAILED. 10561 passed; 1 failed; 105 ignored; 0 measured; 0 filtered out
