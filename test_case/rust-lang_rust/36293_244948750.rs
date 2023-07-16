

failures:

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:295
---- [rustdoc] rustdoc/issue-32374.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "python" "/build/src/etc/htmldocck.py" "/build/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32374.stage2-x86_64-unknown-linux-gnu" "/build/src/test/rustdoc/issue-32374.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
16: @has check failed
    `XPATH PATTERN` did not match
    // @has issue_32374/index.html '//*[@class="docblock short"]' '[Deprecated] [Unstable]'

Encountered 1 errors

------------------------------------------

thread '[rustdoc] rustdoc/issue-32374.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2359
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [rustdoc] rustdoc/issue-32374.rs

test result: FAILED. 120 passed; 1 failed; 1 ignored; 0 measured

