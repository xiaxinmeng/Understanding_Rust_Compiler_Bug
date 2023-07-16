plain
travis_fold:start:worker_info
Worker information
hostname: 2b5a4b35-2d7e-4056-a526-a3b927f0331d@1.production-2-worker-com-gce-btlt
version: v6.2.0 https://github.com/travis-ci/worker/tree/5e5476e01646095f48eec13196fdb3faf8f5cbf7
startup: 6.815506732s
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
---
travis_time:end:2865d998:start=1547137555748345823,finish=1547137556625158916,duration=876813093
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:36] 71  |                           Some(unsafe { $Ty(n) })
[00:04:36]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:36] ...
[00:04:36] 100 | / nonzero_integers! {
[00:04:36] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:36] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:36] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:36] ...   |
[00:04:36] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:36]     | |_- in this macro invocation
[00:04:36]     |
[00:04:36]     = note: #[warn(unused_unsafe)] on by default
[00:04:36] 
[00:04:36] 
[00:04:36] warning: unnecessary `unsafe` block
[00:04:36]    --> src/libcore/num/mod.rs:71:30
[00:04:36]     |
[00:04:36] 71  |                           Some(unsafe { $Ty(n) })
[00:04:36]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:36] ...
[00:04:36] 100 | / nonzero_integers! {
[00:04:36] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:36] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:36] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:36] ...   |
[00:04:36] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:36]     | |_- in this macro invocation
[00:04:36] 
[00:04:36] warning: unnecessary `unsafe` block
[00:04:36]     --> src/libcore/ptr.rs:2785:18
---
[00:04:36] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:36]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:36] ...
[00:04:36] 100 | / nonzero_integers! {
[00:04:36] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:36] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:36] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:36] ...   |
[00:04:36] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:36]     | |_- in this macro invocation
[00:04:36]     |
[00:04:36]     = note: #[warn(unused_attributes)] on by default
[00:04:36] 
[00:04:36] 
[00:04:36] warning: unused attribute
[00:04:36]    --> src/libcore/num/mod.rs:50:17
[00:04:36]     |
[00:04:36] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:36]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:36] ...
[00:04:36] 100 | / nonzero_integers! {
[00:04:36] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:36] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:36] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:36] ...   |
[00:04:36] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:36]     | |_- in this macro invocation
[00:04:36] 
[00:04:36] warning: unused attribute
[00:04:36]     --> src/libcore/ptr.rs:2720:1
---
[01:05:29] .................................................................................................... 4600/5298
[01:05:32] .................................................................................................... 4700/5298
[01:05:35] .................................................................................................... 4800/5298
[01:05:39] .................................................................................................... 4900/5298
[01:05:43] ..............................................F..................................................... 5000/5298
[01:05:49] .................................................................................................... 5200/5298
[01:05:51] .....................................i............................................................
[01:05:51] failures:
[01:05:51] 
[01:05:51] 
[01:05:51] ---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
[01:05:51] diff of stderr:
[01:05:51] 
[01:05:51] 5    |         ^^^^^^^^ the trait `std::convert::From<&str>` is not implemented for `i32`
[01:05:51] 7    = help: the following implementations were found:
[01:05:51] 7    = help: the following implementations were found:
[01:05:51] -              <i32 as std::convert::From<bool>>
[01:05:51] +              <i32 as std::convert::From<core::num::NonZer--------
[01:05:51] {"message":"the trait bound `i32: std::convert::From<&str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n