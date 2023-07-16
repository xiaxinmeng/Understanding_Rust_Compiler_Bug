plain
travis_time:end:14ead65a:start=1550918608983031110,finish=1550918696581639428,duration=87598608318
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:16]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:25]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:11:56]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:11:56]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:11:59] error: unused import: `rustc::ty::cast::CastKind as TyCastKind`
[00:11:59]   --> src/librustc_mir/hair/cx/expr.rs:11:5
[00:11:59] 11 | use rustc::ty::cast::CastKind as TyCastKind;
[00:11:59]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:11:59]    |
[00:11:59]    = note: `-D unused-imports` implied by `-D warnings`
