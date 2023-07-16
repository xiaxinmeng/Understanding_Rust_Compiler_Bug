plain
travis_time:end:0ca3047a:start=1551629717340327631,finish=1551629719469625182,duration=2129297551
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:17]    Compiling test v0.0.0 (/checkout/src/libtest)
[00:05:17] error: stability attributes may not be used outside of the standard library
[00:05:17]   --> src/libtest/lib.rs:12:1
[00:05:17]    |
[00:05:17] 12 | #![unstable(feature = "test", issue = "27812")]
[00:05:17] 
[00:05:17] error: aborting due to previous error
[00:05:17] 
[00:05:17] error: Could not compile `test`.
