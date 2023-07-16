plain
travis_time:end:059deac0:start=1551246108460050611,finish=1551246110614064549,duration=2154013938
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:08]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:12] error[E0432]: unresolved import `fmt`
[00:04:12]   --> src/libcore/convert.rs:44:5
[00:04:12]    |
[00:04:12] 44 | use fmt;
[00:04:12]    |     ^^^ no `fmt` external crate
[00:04:12] 
[00:04:12] error[E0433]: failed to resolve: could not find `mem` in `{{root}}`
[00:04:12]    --> src/libcore/fmt/num.rs:199:27
[00:04:12]     |
[00:04:12] 199 |                   assert!(::mem::size_of::<$u>() >= 2);
[00:04:12]     |                             ^^^ could not find `mem` in `{{root}}`
[00:04:12] 265 | /     impl_Display!(
[00:04:12] 265 | /     impl_Display!(
[00:04:12] 266 | |         i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
[00:04:12] 267 | |             as u64 via to_u64 named fmt_u64
[00:04:12]     | |______- in this macro invocation
[00:04:12] 
[00:04:12] 
[00:04:12] error[E0433]: failed to resolve: could not find `mem` in `{{root}}`
[00:04:12]    --> src/libcore/fmt/num.rs:199:27
[00:04:12]     |
[00:04:12] 199 |                 assert!(::mem::size_of::<$u>() >= 2);
[00:04:12]     |                           ^^^ could not find `mem` in `{{root}}`
[00:04:12] ...
[00:04:12] 278 | impl_Display!(i128, u128 as u128 via to_u128 named fmt_u128);
[00:04:12]     | ------------------------------------------------------------- in this macro invocation
[00:04:12] 
[00:04:13] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:13]    Compiling compiler_builtins v0.1.5
[00:04:13]    Compiling cmake v0.1.33
[00:04:13]    Compiling backtrace-sys v0.1.27
[00:04:16] error: aborting due to 8 previous errors
