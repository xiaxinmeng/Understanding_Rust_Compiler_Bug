
[01:05:24] failures:
[01:05:24] 
[01:05:24] ---- [run-pass] run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs stdout ----
[01:05:24] 
[01:05:24] error: test run failed!
[01:05:24] status: exit code: 101
[01:05:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:05:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment/a"
[01:05:24] stdout:
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:05:24]   left: `"string\r\nliteral"`,
[01:05:24]  right: `"string\nliteral"`', /checkout/src/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs:32:5
[01:05:24] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:24] 
