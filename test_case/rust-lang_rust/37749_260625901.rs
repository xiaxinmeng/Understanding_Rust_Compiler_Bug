
running 3 tests
test test1 ... FAILED
test test2 ... FAILED
test test3 ... ok

failures:

---- test1 stdout ----
    thread 'test1' panicked at 'explicit panic', ./test.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: Panic did not include expected string 'foo'
---- test2 stdout ----
    thread 'test2' panicked at 'bar', ./test.rs:12
note: Panic did not include expected string 'foo'

failures:
    test1
    test2

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured
