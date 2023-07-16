
     Running target/debug/deps/smoke-9c443e6c4ee6f0fc

running 3 tests
test smoke_test_frames ... FAILED
test sp_smoke_test ... ok
test many_threads ... test many_threads has been running for over 60 seconds
test many_threads ... ok

failures:

---- smoke_test_frames stdout ----
thread 'smoke_test_frames' panicked at 'assertion failed: !can_resolve', tests/smoke.rs:180:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
