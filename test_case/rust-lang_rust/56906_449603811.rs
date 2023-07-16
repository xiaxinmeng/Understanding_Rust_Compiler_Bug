plain
travis_time:end:357e14ef:start=1545520155683342708,finish=1545520212127885492,duration=56444542784
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:12:55]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:16] error: variable does not need to be mutable
[00:13:16]    --> src/librustc_mir/build/mod.rs:675:13
[00:13:16]     |
[00:13:16] 675 |         let mut decl = UpvarDecl {
[00:13:16]     |             |
[00:13:16]     |             help: remove this `mut`
[00:13:16]     |
[00:13:16]     = note: `-D unused-mut` implied by `-D warnings`
