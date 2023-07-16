
[01:19:36]    Doc-tests core
[01:19:40] 
[01:19:40] running 1324 tests
<snip>
[01:22:06] failures:
[01:22:06] 
[01:22:06] ---- num/mod.rs - num::u128::leading_zeros (line 107) stdout ----
[01:22:06] 	error: this file contains an un-closed delimiter
[01:22:06]  --> num/mod.rs:114:35
[01:22:06]   |
[01:22:06] 9 | assert_eq!(n.leading_zeros(), 2);
[01:22:06]   |                                   ^
[01:22:06]   |
[01:22:06] help: did you mean to close this delimiter?
[01:22:06]  --> num/mod.rs:111:11
[01:22:06]   |
[01:22:06] 6 | fn main() {
[01:22:06]   |           ^
[01:22:06] 
[01:22:06] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:298:13
[01:22:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:06] 
[01:22:06] 
[01:22:06] failures:
[01:22:06]     num/mod.rs - num::u128::leading_zeros (line 107)
