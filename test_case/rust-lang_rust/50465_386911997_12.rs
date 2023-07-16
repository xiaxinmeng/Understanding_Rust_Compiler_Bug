
[00:44:12]   |     ^
[00:44:12] 
[00:44:12] 
[00:44:12] warning: Backing out of syntax highlighting
[00:44:12]   |
[00:44:12]   = note: You probably did not intend to render this as a rust code-block
[00:44:15]     Finished release [optimized] target(s) in 89.94 secs
[00:44:15] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:44:15]    Compiling getopts v0.2.17
[00:44:15]    Compiling term v0.0.0 (file:///checkout/src/libterm)
---
[01:17:32] ....................................................................................................
[01:17:45] ....................................................................................................
[01:17:58] ....................................................................................................
[01:18:11] ....................................................................................................
[01:18:22] ..................................FF..FF...FF.F....FF.F..F.FFF.....FF.F..F.FF.F....F.FF..F.FF.F....F
[01:18:32] .FF..FFF..F.....FFF..FFF.F......FFF...F.F...F.F.....FF....FF......FF...F..F.....FF...F..F.....FF...F
[01:18:45] .F......FF..........................................................................................
[01:19:14] ....................................................................................................
[01:19:30] ....................................................................................................
[01:19:45] ....................................................................................................
[01:20:07] ....................................................................................................
---
[01:20:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:20:19] 
[01:20:19] ', librustdoc/test.rs:356:17
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i128>::leading_zeros (line 205) stdout ----
[01:20:19]  error[E0369]: binary operation `>>` cannot be applied to type `std::num::Wrapping<i128>`
[01:20:19]  --> num/mod.rs:209:9
[01:20:19]   |
[01:20:19] 7 | let n = Wrapping(i128::max_value()) >> 2;
[01:20:19]   |
[01:20:19]   |
[01:20:19]   = note: an implementation of `std::ops::Shr` might be missing for `std::num::Wrapping<i128>`
[01:20:19] thread 'num/mod.rs - num::wrapping::Wrapping<i128>::leading_zeros (line 205)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:20:19] 
[01:20:19] ---- num/mod.rs - num::wrapping::Wrapping<i128>::signum (line 209) stdout ----
[01:20:19]  error: unknown start of token: `
[01:20:19]  error: unknown start of token: `
[01:20:19]   --> num/mod.rs:216:5
[01:20:19]    |
[01:20:19] 10 |     