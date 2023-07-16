plain
travis_time:end:00fd75e6:start=1554484533056697214,finish=1554484619062687760,duration=86005990546
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:31]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:33] error: hidden lifetime parameters in types are deprecated
[00:04:33]    --> src/libstd/panic.rs:326:44
[00:04:33]     |
[00:04:33] 326 |     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
[00:04:33] 
[00:04:35] error: aborting due to previous error
[00:04:35] 
[00:04:35] error: Could not compile `std`.
