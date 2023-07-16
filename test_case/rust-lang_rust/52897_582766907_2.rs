
$ rustc --version
rustc 1.41.0 (5e1a79984 2020-01-27)
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running target/debug/deps/floatextremestest-23756e6931b17aa8

running 2 tests
test tests::max ... FAILED
test tests::min ... FAILED

failures:

---- tests::max stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `NaN`,
 right: `1.0`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

---- tests::min stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `NaN`,
 right: `1.0`', src/lib.rs:6:9


failures:
    tests::max
    tests::min

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
