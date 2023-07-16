

failures:

---- src\process.rs - process::Command (line 461) stdout ----
Test executable failed (exit code: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"hello\r\n"`,
 right: `"hello\n"`', src\process.rs:19:1                           <--- This line
stack backtrace:
