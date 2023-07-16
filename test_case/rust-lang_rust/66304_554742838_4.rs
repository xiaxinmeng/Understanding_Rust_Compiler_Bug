
running 1 test
test assert_eq_shell_escape_test ... FAILED

failures:

---- assert_eq_shell_escape_test stdout ----
thread 'assert_eq_shell_escape_test' panicked at 'assertion failed: `(left == right)`
  left: `"foo\u{1b}[1Dbar"`,
 right: `"fobar"`', src/lib.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    assert_eq_shell_escape_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
