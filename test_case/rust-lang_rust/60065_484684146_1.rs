
running 1 test
test issue-59313.rs -  (line 1) ... FAILED

failures:

---- issue-59313.rs -  (line 1) stdout ----
error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `move`
 --> issue-59313.rs:5:16
  |
6 |     drop(async move {});
  |                ^^^^ expected one of 8 possible tokens here

error: aborting due to previous error

thread 'issue-59313.rs -  (line 1)' panicked at 'couldn't compile the test', src\librustdoc\test.rs:310:13
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    issue-59313.rs -  (line 1)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
