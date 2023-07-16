plain
travis_time:end:02bc0f5c:start=1543798313634023233,finish=1543798316033782501,duration=2399759268
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:18]    Compiling crossbeam-deque v0.2.0
[00:05:21]    Compiling syn v0.15.21
[00:05:21]    Compiling rustc-rayon v0.1.1
[00:05:37]    Compiling synstructure v0.10.1
[00:05:45]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[00:05:51]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:51]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:56]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:22]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:22]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:24]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:24]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:31] error[E0277]: the trait bound `rustc::ty::Predicate<'_>: rustc_data_structures::local_drop::LocalDrop` is not satisfied
[00:13:31]    --> src/librustc_typeck/outlives/mod.rs:121:26
[00:13:31]     |
[00:13:31] 121 |             (def_id, tcx.promote_vec(vec))
[00:13:31]     |                          ^^^^^^^^^^^ the trait `rustc_data_structures::local_drop::LocalDrop` is not implemented for `rustc::ty::Predicate<'_>`
[00:13:31] error: aborting due to previous error
[00:13:31] 
[00:13:31] For more information about this error, try `rustc --explain E0277`.
[00:13:31] error: Could not compile `rustc_typeck`.
