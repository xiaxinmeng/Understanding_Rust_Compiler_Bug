
---- src/lib.rs - foo (line 1) stdout ----
Test executable failed (exit code 101).
stdout:
baz(0, 128, 4) -> (128, true)
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `128`,
 right: `8`', src/lib.rs:4:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
