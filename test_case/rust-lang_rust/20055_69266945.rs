
test [run-pass] run-pass/zero-size-type-destructors.rs ... ok
test [run-pass] run-pass/yield1.rs ... ok
test [run-pass] run-pass/yield.rs ... ok
test [run-pass] run-pass/vector-sort-panic-safe.rs ... ok

failures:

---- [run-pass] run-pass/coerce-match-calls.rs stdout ----

error: test run failed!
status: signal: 4
command: x86_64-apple-darwin/test/run-pass/coerce-match-calls.stage2-x86_64-apple-darwin
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[run-pass] run-pass/coerce-match-calls.rs' panicked at 'explicit panic', /Users/jroesch/Git/rust/src/compiletest/runtest.rs:1453


---- [run-pass] run-pass/coerce-match.rs stdout ----

error: test run failed!
status: signal: 4
command: x86_64-apple-darwin/test/run-pass/coerce-match.stage2-x86_64-apple-darwin
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[run-pass] run-pass/coerce-match.rs' panicked at 'explicit panic', /Users/jroesch/Git/rust/src/compiletest/runtest.rs:1453



failures:
    [run-pass] run-pass/coerce-match-calls.rs
    [run-pass] run-pass/coerce-match.rs

test result: FAILED. 1826 passed; 2 failed; 28 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /Users/jroesch/Git/rust/src/compiletest/compiletest.rs:269
make: *** [tmp/check-stage2-T-x86_64-apple-darwin-H-x86_64-apple-darwin-rpass.ok] Error 101
