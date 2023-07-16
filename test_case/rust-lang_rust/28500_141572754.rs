

failures:

---- [run-pass] run-pass/core-run-destroy.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/core-run-destroy.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

running 1 test
test test_destroy_actually_kills ... FAILED

failures:

---- test_destroy_actually_kills stdout ----
    thread 'test_destroy_actually_kills' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 13, message: "Permission denied" } }', src/libcore/result.rs:736



failures:
    test_destroy_actually_kills

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured


------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[run-pass] run-pass/core-run-destroy.rs' panicked at 'explicit panic', /build/src/compiletest/runtest.rs:1501


---- [run-pass] run-pass/issue-26468.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/issue-26468.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: `(left == right)` (left: `42`, right: `19`)', /build/src/test/run-pass/issue-26468.rs:37

------------------------------------------

thread '[run-pass] run-pass/issue-26468.rs' panicked at 'explicit panic', /build/src/compiletest/runtest.rs:1501



failures:
    [run-pass] run-pass/core-run-destroy.rs
    [run-pass] run-pass/issue-26468.rs
