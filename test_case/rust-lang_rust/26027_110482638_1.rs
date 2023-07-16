
failures:

---- [run-fail] run-fail/test-panic.rs stdout ----

error: no more error patterns to match against
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-fail/test-panic.stage1-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

running 1 test
test test_foo ... FAILED

failures:

---- test_foo stdout ----
    thread 'test_foo' panicked at 'explicit panic', /home/nham/code/other/rust2/src/test/run-fail/test-panic.rs:17



failures:
    test_foo

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured


------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'Some tests failed', /home/nham/code/other/rust2/src/libtest/lib.rs:255

------------------------------------------

thread '[run-fail] run-fail/test-panic.rs' panicked at 'explicit panic', /home/nham/code/other/rust2/src/compiletest/runtest.rs:1521
