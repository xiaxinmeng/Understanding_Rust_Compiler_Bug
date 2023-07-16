console
$ rustdoc +nightly-2018-05-13 --test j.rs

running 1 test
test j.rs - SomeStruct (line 1) ... FAILED

failures:

---- j.rs - SomeStruct (line 1) stdout ----
        thread 'j.rs - SomeStruct (line 1)' panicked at 'test executable failed:

thread 'main' panicked at 'oh no', j.rs:3:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', librustdoc/test.rs:356:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    j.rs - SomeStruct (line 1)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

$ rustdoc +nightly-2018-05-14 --test j.rs

running 1 test
test j.rs - SomeStruct (line 1) ... FAILED

failures:

failures:
    j.rs - SomeStruct (line 1)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
