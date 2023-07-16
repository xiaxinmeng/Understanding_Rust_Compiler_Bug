
     Running target/debug/deps/accuracy-18628a8819e01c2c

running 1 test
test doit ... FAILED

failures:

---- doit stdout ----
-----------------------------------
looking for:
        tests/accuracy/main.rs:40
        crates/dylib-dep/src/lib.rs:11
        tests/accuracy/main.rs:39
found:
   0: <unknown>
   1: <unknown>
   2: <unknown>
   3: <unknown>
   4: <unknown>
   5: <unknown>
   6: <unknown>
   7: <unknown>
   8: <unknown>
   9: <unknown>
  10: <unknown>
  11: <unknown>
  12: <unknown>
  13: <unknown>
  14: <unknown>
  15: <unknown>
  16: start_thread
  17: <unknown>

thread 'doit' panicked at 'failed to find tests/accuracy/main.rs:40', tests/accuracy/main.rs:104:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
