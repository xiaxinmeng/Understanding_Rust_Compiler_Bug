plain
travis_time:end:08b7c5dd:start=1550604475419194230,finish=1550604547800847383,duration=72381653153
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:15] tidy error: /checkout/src/libcore/pin.rs:196: unexplained "