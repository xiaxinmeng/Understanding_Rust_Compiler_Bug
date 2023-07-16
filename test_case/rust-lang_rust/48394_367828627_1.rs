console
$ rustc +nightly --test e.rs
$ ./e

running 1 test
test bar ... FAILED

failures:

---- bar stdout ----
        thread 'bar' panicked at 'the disco', e.rs:10:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    bar

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
