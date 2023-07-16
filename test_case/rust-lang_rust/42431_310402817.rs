
[01:11:14] failures:
[01:11:14] 
[01:11:14] ---- f32.rs - f32::f32::is_sign_negative (line 388) stdout ----
[01:11:14] 	thread 'rustc' panicked at 'test executable failed:
[01:11:14] 
[01:11:14] thread 'main' panicked at 'assertion failed: !nan.is_sign_positive() && !nan.is_sign_negative()', <anon>:13
[01:11:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:14] 
[01:11:14] ', /checkout/src/librustdoc/test.rs:318
[01:11:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:14] 
[01:11:14] ---- f32.rs - f32::f32::is_sign_positive (line 369) stdout ----
[01:11:14] 	thread 'rustc' panicked at 'test executable failed:
[01:11:14] 
[01:11:14] thread 'main' panicked at 'assertion failed: !nan.is_sign_positive() && !nan.is_sign_negative()', <anon>:13
[01:11:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:14] 
[01:11:14] ', /checkout/src/librustdoc/test.rs:318
[01:11:14] 
[01:11:14] ---- f64.rs - f64::f64::is_sign_negative (line 332) stdout ----
[01:11:14] 	thread 'rustc' panicked at 'test executable failed:
[01:11:14] 
[01:11:14] thread 'main' panicked at 'assertion failed: !nan.is_sign_positive() && !nan.is_sign_negative()', <anon>:14
[01:11:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:14] 
[01:11:14] ', /checkout/src/librustdoc/test.rs:318
[01:11:14] 
[01:11:14] ---- f64.rs - f64::f64::is_sign_positive (line 307) stdout ----
[01:11:14] 	thread 'rustc' panicked at 'test executable failed:
[01:11:14] 
[01:11:14] thread 'main' panicked at 'assertion failed: !nan.is_sign_positive() && !nan.is_sign_negative()', <anon>:14
[01:11:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:14] 
[01:11:14] ', /checkout/src/librustdoc/test.rs:318
[01:11:14] 
[01:11:14] 
[01:11:14] failures:
[01:11:14]     f32.rs - f32::f32::is_sign_negative (line 388)
[01:11:14]     f32.rs - f32::f32::is_sign_positive (line 369)
[01:11:14]     f64.rs - f64::f64::is_sign_negative (line 332)
[01:11:14]     f64.rs - f64::f64::is_sign_positive (line 307)
[01:11:14] 
[01:11:14] test result: FAILED. 820 passed; 4 failed; 22 ignored; 0 measured; 0 filtered out
