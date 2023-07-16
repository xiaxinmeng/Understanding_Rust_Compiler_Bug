plain
travis_time:end:05a5cd00:start=1556820342676309860,finish=1556820343451738729,duration=775428869
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:59] tidy error: /checkout/src/librustc/lint/context.rs:761: unexplained "