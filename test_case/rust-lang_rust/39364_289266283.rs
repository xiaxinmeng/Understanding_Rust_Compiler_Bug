rust
DEBUG:rfail: - - - - - - - - - - - - - - - - - - - -
DEBUG:rfail: >>>>> 716 >>>>>
DEBUG:rfail: 716 - found : 0
DEBUG:rfail: 716 - request -> 0
DEBUG:rfail: 716 - response <- 0
DEBUG:rfail: 716 - found : 1
DEBUG:rfail: 716 - request -> 1
DEBUG:rfail: 716 - response <- 1
DEBUG:rfail: 716 - found : 2
DEBUG:rfail: 716 - request -> 2
DEBUG:rfail: 716 - response <- 2
DEBUG:rfail: <<<<< 716 <<<<<
DEBUG:rfail: - - - - - - - - - - - - - - - - - - - -
DEBUG:rfail: >>>>> 717 >>>>>
DEBUG:rfail: 717 - found : 0
thread '<unnamed>' panicked at 'assertion failed: `(left == right)` (left: `139883544888960`, right: `0`)', /checkout/src/libstd/sync/mpsc/shared.rs:253
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread '<unnamed>' panicked at 'receive result: RecvError', /checkout/src/libcore/result.rs:859
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any', /checkout/src/libcore/result.rs:859
