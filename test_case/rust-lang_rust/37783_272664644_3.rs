
$ RUST_BACKTRACE=test cargo test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/action-80d4ecacee9f9bcc

running 4 tests
test license ... ok
test parser ... ok
test tostr ... ok
test errors ... FAILED

failures:

---- errors stdout ----
	thread 'errors' panicked at 'Action not invalid: depend type=require', /builds/rsdev/tests/utils/mod.rs:105
stack backtrace:
   1:     0x560d0db4550b - action::utils::assert_invalid::h8e7640be05700633
                        at /builds/rsdev/tests/utils/mod.rs:105
   2:     0x560d0db4cb86 - action::errors::h54693ccbd5897fb6
                        at /builds/rsdev/tests/action.rs:389
