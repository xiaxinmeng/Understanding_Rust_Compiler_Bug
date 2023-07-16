
running 1 test
test /dev/fd/63 - main (line 1) ... FAILED

failures:

---- /dev/fd/63 - main (line 1) stdout ----
	thread 'rustc' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: `(left == right)` (left: `10`, right: `5`)', <anon>:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', /checkout/src/librustdoc/test.rs:313
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    /dev/fd/63 - main (line 1)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
