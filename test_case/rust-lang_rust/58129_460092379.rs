plain
travis_time:end:02cfd959:start=1549228639999197619,finish=1549228716183123539,duration=76183925920
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:26] tidy error: /checkout/src/libcore/mem.rs:1045: unexplained "