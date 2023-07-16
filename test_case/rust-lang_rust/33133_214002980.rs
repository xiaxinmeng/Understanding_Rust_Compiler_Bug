
failures:

---- [rustdoc] rustdoc/inline_local/issue-32343.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/home/travis/build/rust-lang/rust/src/etc/htmldocck.py" "x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-32343.stage2-x86_64-unknown-linux-gnu" "/home/travis/build/rust-lang/rust/src/test/rustdoc/inline_local/issue-32343.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
21: @has check failed
    `XPATH PATTERN` did not match
    // @has - '//code/a' 'Bar'

Encountered 1 errors

------------------------------------------

thread '[rustdoc] rustdoc/inline_local/issue-32343.rs' panicked at 'explicit panic', /home/travis/build/rust-lang/rust/src/tools/compiletest/src/runtest.rs:1597
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [rustdoc] rustdoc/inline_local/issue-32343.rs
