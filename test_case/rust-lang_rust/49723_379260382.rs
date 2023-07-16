console
$ rustdoc +nightly-2018-03-21 --test h.rs

running 1 test
test h.rs - foo (line 7) ... FAILED

failures:

---- h.rs - foo (line 7) stdout ----
        thread 'rustc' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: false', h.rs:3:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', librustdoc/test.rs:331:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    h.rs - foo (line 7)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

$ rustdoc +nightly-2018-03-24 --test h.rs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
