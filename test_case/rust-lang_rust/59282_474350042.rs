plain
travis_time:end:0ad8dad4:start=1552995036480967131,finish=1552995038796648969,duration=2315681838
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:end:00429b30:start=1553000007044012407,finish=1553000007048861776,duration=4849369
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e05832f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj
