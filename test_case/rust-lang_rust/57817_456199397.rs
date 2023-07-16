plain
travis_time:end:17a19948:start=1548102614365033087,finish=1548102692808555072,duration=78443521985
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:25] tidy error: /checkout/src/libsyntax/parse/parser.rs:2792: unexplained "