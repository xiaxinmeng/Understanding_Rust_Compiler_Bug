
$ cargo test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/action-80d4ecacee9f9bcc

running 4 tests
test license ... ok
test errors ... FAILED
test parser ... ok
test tostr ... ok

failures:

---- errors stdout ----
	thread 'errors' panicked at 'Action not invalid: depend type=require', /builds/rsdev/tests/utils/mod.rs:105
note: Run with `RUST_BACKTRACE=1` for a backtrace.
...
