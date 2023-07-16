plain
[01:15:10] .................................................................................................... 1800/2164
[01:15:18] .................................................................................................... 1900/2164
[01:15:26] .................................................................................................... 2000/2164
[01:15:37] ..................................................................................i................. 2100/2164
[01:15:41] .....................................i...........FF......F......
[01:15:41] 
[01:15:41] ---- time.rs - time::Duration::div_duration (line 529) stdout ----
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:530:12
[01:15:41]  --> time.rs:530:12
[01:15:41]   |
[01:15:41] 4 | let dur1 = Duration::new(2, 700_000_000);
[01:15:41]   |            ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:531:12
[01:15:41]   |
[01:15:41]   |
[01:15:41] 5 | let dur2 = Duration::new(5, 400_000_000);
[01:15:41]   |            ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] thread 'time.rs - time::Duration::div_duration (line 529)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:15:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:15:41] 
[01:15:41] 
[01:15:41] ---- time.rs - time::Duration::div_f64 (line 497) stdout ----
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:498:11
[01:15:41]   |
[01:15:41] 4 | let dur = Duration::new(2, 700_000_000);
[01:15:41]   |           ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:499:31
[01:15:41]   |
[01:15:41]   |
[01:15:41] 5 | assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
[01:15:41]   |                               ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:501:33
[01:15:41]   |
[01:15:41]   |
[01:15:41] 7 | assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_598));
[01:15:41]   |                                 ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] 
[01:15:41] thread 'time.rs - time::Duration::div_f64 (line 497)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:15:41] 
[01:15:41] ---- time.rs - time::Duration::mul_f64 (line 466) stdout ----
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:467:11
[01:15:41]   |
[01:15:41] 4 | let dur = Duration::new(2, 700_000_000);
[01:15:41]   |           ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:468:31
[01:15:41]   |
[01:15:41]   |
[01:15:41] 5 | assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
[01:15:41]   |                               ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] error[E0433]: failed to resolve. Use of undeclared type or module `Duration`
[01:15:41]  --> time.rs:469:33
[01:15:41]   |
[01:15:41]   |
[01:15:41] 6 | assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
[01:15:41]   |                                 ^^^^^^^^ Use of undeclared type or module `Duration`
[01:15:41] 
[01:15:41] thread 'time.rs - time::Duration::mul_f64 (line 466)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:15:41] 
[01:15:41] failures:
[01:15:41]     time.rs - time::Duration::div_duration (line 529)
[01:15:41]     time.rs - time::Duration::div_f64 (line 497)
