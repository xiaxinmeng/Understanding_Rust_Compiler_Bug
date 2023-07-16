
[01:17:24] [1m[31merror: internal compiler error(B[m[1m: librustc_trans/mir/block.rs:838: can't directly store to unaligned value(B[m
[01:17:24]   (B[m[1m[34m--> (B[m/cargo/registry/src/github.com-1ecc6299db9ec823/failure-0.1.1/src/error.rs:31:5(B[m
[01:17:24]    (B[m[1m[34m|(B[m
[01:17:24] [1m[34m31(B[m (B[m[1m[34m| (B[m[1m[31m/(B[m (B[m    fn from(failure: F) -> Error {(B[m
[01:17:24] [1m[34m32(B[m (B[m[1m[34m| (B[m[1m[31m|(B[m (B[m        let inner: Inner<F> = {(B[m
[01:17:24] [1m[34m33(B[m (B[m[1m[34m| (B[m[1m[31m|(B[m (B[m            let backtrace = if failure.backtrace().is_none() {(B[m
[01:17:24] [1m[34m34(B[m (B[m[1m[34m| (B[m[1m[31m|(B[m (B[m                Backtrace::new()(B[m
[01:17:24] [1m[34m...(B[m  (B[m[1m[31m|(B[m
[01:17:24] [1m[34m38(B[m (B[m[1m[34m| (B[m[1m[31m|(B[m (B[m        Error { inner: Box::new(inner) }(B[m
[01:17:24] [1m[34m39(B[m (B[m[1m[34m| (B[m[1m[31m|(B[m (B[m    }(B[m
[01:17:24]    (B[m[1m[34m| (B[m[1m[31m|_____^(B[m
[01:17:24] 
[01:17:24] thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:482:9
[01:17:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:24] 
[01:17:24] note: the compiler unexpectedly panicked. this is a bug.
[01:17:24] 
[01:17:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:24] 
[01:17:24] note: rustc 1.25.0-nightly (7a4d4c6c9 2018-02-01) running on x86_64-unknown-linux-gnu
[01:17:24] 
[01:17:24] [m[m[31m[1merror:[m Could not compile `cargo`.
[01:17:24] 
