plain
travis_time:end:1015899a:start=1548542952022294535,finish=1548543030269218625,duration=78246924090
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:12] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:12] tidy error: /checkout/src/librustc/middle/privacy.rs:21: unexplained "