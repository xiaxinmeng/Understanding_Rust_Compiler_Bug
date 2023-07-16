
[01:12:33]    Doc-tests core
[01:12:36] 
[01:12:36] running 1324 tests
<snip>
[01:14:46] failures:
[01:14:46] 
[01:14:46] ---- num/mod.rs - num::u128::leading_zeros (line 107) stdout ----
[01:14:46] 	error[E0658]: 128-bit type is unstable (see issue #35118)
[01:14:46]  --> num/mod.rs:108:9
[01:14:46]   |
[01:14:46] 4 | let n = u128::max_value() >> 2;
[01:14:46]   |         ^^^^^^^^^^^^^^^
[01:14:46]   |
[01:14:46]   = help: add #![feature(i128_type)] to the crate attributes to enable
[01:14:46] 
[01:14:46] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:298:13
[01:14:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:46] 
[01:14:46] 
[01:14:46] failures:
[01:14:46]     num/mod.rs - num::u128::leading_zeros (line 107)
