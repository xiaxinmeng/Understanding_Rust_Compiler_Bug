plain
travis_time:end:0a06bba8:start=1544804643926193860,finish=1544804698271149349,duration=54344955489
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:44]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:03:49] warning: function is never used: `deinit`
[00:03:49]    --> src/libstd/sys/unix/thread.rs:358:5
[00:03:49]     |
[00:03:49] 358 |     pub unsafe fn deinit() {
[00:03:49]     |
[00:03:49]     = note: #[warn(dead_code)] on by default
[00:03:49] 
[00:04:00]     Finished release [optimized] target(s) in 52.73s
