plain
[00:04:01]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:02]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:03]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:04]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:05] error[E0277]: cannot divide `f64` by `time::Duration`
[00:04:05]     |
[00:04:05]     |
[00:04:05] 621 |         let nanos1 = (NPS * (self.secs as f64) + (self.nanos as f64)) / rhs;
[00:04:05]     |                                                                       ^ no implementation for `f64 / time::Duration`
[00:04:05]     |
[00:04:05]     = help: the trait `ops::arith::Div<time::Duration>` is not implemented for `f64`
[00:04:05] 
[00:04:05] error[E0277]: cannot divide `f64` by `time::Duration`
[00:04:05]     |
[00:04:05]     |
[00:04:05] 622 |         let nanos2 = (NPS * (self.secs as f64) + (self.nanos as f64)) / rhs;
[00:04:05]     |                                                                       ^ no implementation for `f64 / time::Duration`
[00:04:05]     |
[00:04:05]     = help: the trait `ops::arith::Div<time::Duration>` is not implemented for `f64`
[00:04:05] error: aborting due to 2 previous errors
[00:04:05] 
[00:04:05] For more information about this error, try `rustc --explain E0277`.
[00:04:05] error: Could not compile `core`.
