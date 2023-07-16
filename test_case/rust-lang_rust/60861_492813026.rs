plain
travis_time:end:113691a8:start=1557949112234241692,finish=1557949198712449055,duration=86478207363
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:52:54]    Compiling rand_isaac v0.1.1
[00:52:54]    Compiling rand_hc v0.1.0
[00:52:54]    Compiling rand_xorshift v0.1.0
[00:52:55]    Compiling minifier v0.0.30
[00:52:55] error: local ambiguity: multiple parsing options: built-in NTs expr ('else_cond') or 1 other option.
[00:52:55]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.30/src/css/token.rs:564:17
[00:52:55]     |
[00:52:55] 564 |                 let Ok(op) = Operator::try_from(c) => {
[00:52:55] 
[00:52:55] error: aborting due to previous error
[00:52:55] 
[00:52:55] error: Could not compile `minifier`.
