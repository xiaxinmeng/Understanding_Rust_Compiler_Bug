
running 1 test
test shell_escape_panic_test ... FAILED

failures:

---- shell_escape_panic_test stdout ----
thread 'shell_escape_panic_test' panicked at 'fobar', src/lib.rs:4:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
note: panic did not include expected string 'fobar'

failures:
    shell_escape_panic_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
