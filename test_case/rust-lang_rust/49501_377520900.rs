
[01:15:04] ---- time.rs - time::SystemTime::UNIX_EPOCH (line 273) stdout ----
[01:15:04] 	error[E0658]: use of unstable library feature 'assoc_unix_epoch' (see issue #49502)
[01:15:04]  --> time.rs:276:40
[01:15:04]   |
[01:15:04] 6 | match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
[01:15:04]   |                                        ^^^^^^^^^^^^^^^^^^^^^^
[01:15:04]   |
[01:15:04]   = help: add #![feature(assoc_unix_epoch)] to the crate attributes to enable
[01:15:04] 
[01:15:04] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:296:13
[01:15:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:15:04] 
[01:15:04] 
[01:15:04] failures:
[01:15:04]     time.rs - time::SystemTime::UNIX_EPOCH (line 273)
[01:15:04] 
[01:15:04] test result: FAILED. 921 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
