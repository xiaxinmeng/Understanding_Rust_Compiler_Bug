

---- thread::panicking_0 stdout ----
    thread 'thread::panicking_0' panicked at 'test executable failed:
a: dropped while not unwinding
b: dropped while unwinding

thread 'main' panicked at 'explicit panic', <anon>:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.
', src/librustdoc/test.rs:312
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    thread::panicking_0

test result: FAILED. 475 passed; 1 failed; 14 ignored; 0 measured
