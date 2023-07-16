plain
travis_time:end:155bb11f:start=1560709549162357296,finish=1560709638422250406,duration=89259893110
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:57] tidy error: /checkout/src/librustc_typeck/check/wfcheck.rs:380: unexplained "