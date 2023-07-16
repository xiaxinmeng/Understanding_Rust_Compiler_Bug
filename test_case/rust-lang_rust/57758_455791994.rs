plain
travis_time:end:014fb0b8:start=1547913143193130095,finish=1547913219971756506,duration=76778626411
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:26]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:26] error: this file contains an un-closed delimiter
[00:05:26]   --> src/librustc_target/spec/powerpc64_unknown_freebsd.rs:21:8
[00:05:26]    |
[00:05:26] 3  | pub fn target() -> TargetResult {
[00:05:26]    |                                 - un-closed delimiter
[00:05:26] 21 |     })
[00:05:26]    |        ^
[00:05:26] 
[00:05:26] error: aborting due to previous error
