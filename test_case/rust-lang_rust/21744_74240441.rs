 rust
failures:

---- [run-pass] run-pass/deref-rc.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/deref-rc.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: *x == [1, 2, 3, 4]', /Users/eddy/Documents/rust/src/test/run-pass/deref-rc.rs:15

------------------------------------------

thread '[run-pass] run-pass/deref-rc.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/issue-15080.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/issue-15080.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: result == [2, 4]', /Users/eddy/Documents/rust/src/test/run-pass/issue-15080.rs:29

------------------------------------------

thread '[run-pass] run-pass/issue-15080.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/overloaded-deref-count.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/overloaded-deref-count.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: `(left == right) && (right == left)` (left: `[1, 2]`, right: `[1, 2]`)', /Users/eddy/Documents/rust/src/test/run-pass/overloaded-deref-count.rs:85

------------------------------------------

thread '[run-pass] run-pass/overloaded-deref-count.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/packed-struct-generic-layout.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/packed-struct-generic-layout.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: transd == [1, 2, 129, 129]', /Users/eddy/Documents/rust/src/test/run-pass/packed-struct-generic-layout.rs:34

------------------------------------------

thread '[run-pass] run-pass/packed-struct-generic-layout.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/packed-struct-layout.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/packed-struct-layout.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: transd == [1, 2, 3, 4]', /Users/eddy/Documents/rust/src/test/run-pass/packed-struct-layout.rs:29

------------------------------------------

thread '[run-pass] run-pass/packed-struct-layout.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/packed-tuple-struct-layout.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/packed-tuple-struct-layout.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: transd == [1, 2, 3, 4]', /Users/eddy/Documents/rust/src/test/run-pass/packed-tuple-struct-layout.rs:23

------------------------------------------

thread '[run-pass] run-pass/packed-tuple-struct-layout.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/vec-matching-autoslice.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/vec-matching-autoslice.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: [a, b] == [2, 3]', /Users/eddy/Documents/rust/src/test/run-pass/vec-matching-autoslice.rs:16

------------------------------------------

thread '[run-pass] run-pass/vec-matching-autoslice.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452


---- [run-pass] run-pass/vec-matching.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/vec-matching.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: `(left == right) && (right == left)` (left: `[2]`, right: `[2]`)', /Users/eddy/Documents/rust/src/test/run-pass/vec-matching.rs:44

------------------------------------------

thread '[run-pass] run-pass/vec-matching.rs' panicked at 'explicit panic', /Users/eddy/Documents/rust/src/compiletest/runtest.rs:1452



failures:
    [run-pass] run-pass/deref-rc.rs
    [run-pass] run-pass/issue-15080.rs
    [run-pass] run-pass/overloaded-deref-count.rs
    [run-pass] run-pass/packed-struct-generic-layout.rs
    [run-pass] run-pass/packed-struct-layout.rs
    [run-pass] run-pass/packed-tuple-struct-layout.rs
    [run-pass] run-pass/vec-matching-autoslice.rs
    [run-pass] run-pass/vec-matching.rs

test result: FAILED. 1909 passed; 8 failed; 38 ignored; 0 measured
