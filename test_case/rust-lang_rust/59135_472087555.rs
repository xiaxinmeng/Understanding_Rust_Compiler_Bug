plain
travis_time:end:06917274:start=1552408772863483811,finish=1552408865094412188,duration=92230928377
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:17] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:17] tidy error: /checkout/src/librustc/hir/mod.rs:1569: unexplained "