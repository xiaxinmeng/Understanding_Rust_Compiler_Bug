plain
[00:03:25]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:26] warning: unused variable: `state`
[00:03:26]    --> libcore/hash/mod.rs:608:35
[00:03:26]     |
[00:03:26] 608 |         fn hash<H: Hasher>(&self, state: &mut H) {
[00:03:26]     |                                   ^^^^^ help: consider using `_state` instead
[00:03:26]     = note: #[warn(unused_variables)] on by default
[00:03:26] 
[00:03:27]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:39]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
