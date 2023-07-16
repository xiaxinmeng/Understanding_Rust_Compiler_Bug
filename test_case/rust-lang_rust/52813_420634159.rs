plain
[01:14:17] .................................................................................................... 1800/2164
[01:14:25] .................................................................................................... 1900/2164
[01:14:34] .................................................................................................... 2000/2164
[01:14:45] ..................................................................................i................. 2100/2164
[01:14:50] .....................................i............F.F....F......
[01:14:50] 
[01:14:50] ---- time.rs - time::Duration::div_duration (line 533) stdout ----
[01:14:50] ---- time.rs - time::Duration::div_duration (line 533) stdout ----
[01:14:50] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:14:50]  --> time.rs:538:17
[01:14:50]   |
[01:14:50] 8 | assert_eq!(dur1.div_duration(dur2), 0.5);
[01:14:50]   |
[01:14:50]   |
[01:14:50]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:14:50] thread 'time.rs - time::Duration::div_duration (line 533)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:14:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:50] 
[01:14:50] 
[01:14:50] ---- time.rs - time::Duration::div_f64 (line 499) stdout ----
[01:14:50] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:14:50]  --> time.rs:503:16
[01:14:50]   |
[01:14:50] 7 | assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
[01:14:50]   |
[01:14:50]   |
[01:14:50]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:14:50] 
[01:14:50] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:14:50]  --> time.rs:505:16
[01:14:50]   |
[01:14:50] 9 | assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_598));
[01:14:50]   |
[01:14:50]   |
[01:14:50]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:14:50] 
[01:14:50] thread 'time.rs - time::Duration::div_f64 (line 499)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:14:50] 
[01:14:50] ---- time.rs - time::Duration::mul_f64 (line 466) stdout ----
[01:14:50] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:14:50]  --> time.rs:470:16
[01:14:50]   |
[01:14:50] 7 | assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
[01:14:50]   |
[01:14:50]   |
[01:14:50]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:14:50] 
[01:14:50] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:14:50]  --> time.rs:471:16
[01:14:50]   |
[01:14:50] 8 | assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
[01:14:50]   |
[01:14:50]   |
[01:14:50]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:14:50] 
[01:14:50] thread 'time.rs - time::Duration::mul_f64 (line 466)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:14:50] 
[01:14:50] failures:
[01:14:50]     time.rs - time::Duration::div_duration (line 533)
[01:14:50]     time.rs - time::Duration::div_f64 (line 499)
