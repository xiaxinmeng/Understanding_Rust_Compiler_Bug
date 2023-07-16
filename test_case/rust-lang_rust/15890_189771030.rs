
$ rustc $'\xff'
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: "\u{fffd}"', ../src/libcore/result.rs:746
note: Run with `RUST_BACKTRACE=1` for a backtrace.
