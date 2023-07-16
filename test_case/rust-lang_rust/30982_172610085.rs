
failures:

---- [run-pass] run-pass/zero-sized-btreemap-insert.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/zero-sized-btreemap-insert.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', /home/travis/build/rust-lang/rust/src/test/run-pass/zero-sized-btreemap-insert.rs:40

------------------------------------------

thread '[run-pass] run-pass/zero-sized-btreemap-insert.rs' panicked at 'explicit panic', /home/travis/build/rust-lang/rust/src/compiletest/runtest.rs:1505

---- [run-pass] run-pass/zero-sized-string-push.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/zero-sized-string-push.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: `(left == right)` (left: `0`, right: `1`)', /home/travis/build/rust-lang/rust/src/test/run-pass/zero-sized-string-push.rs:28

------------------------------------------

thread '[run-pass] run-pass/zero-sized-string-push.rs' panicked at 'explicit panic', /home/travis/build/rust-lang/rust/src/compiletest/runtest.rs:1505


failures:
    [run-pass] run-pass/zero-sized-btreemap-insert.rs
    [run-pass] run-pass/zero-sized-string-push.rs

test result: [31mFAILED(B[0m. 2351 passed; 2 failed; 2 ignored; 0 measured
