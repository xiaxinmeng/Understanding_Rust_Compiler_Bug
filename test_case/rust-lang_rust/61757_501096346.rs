plain
travis_time:end:07b6d120:start=1560304861869235752,finish=1560304863831939339,duration=1962703587
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:47]    Compiling hashbrown v0.4.0
[00:27:50] error: use of deprecated item 'sync::once::ONCE_INIT': the `new` function is now preferred
[00:27:50]    --> src/libstd/sync/mod.rs:166:39
[00:27:50]     |
[00:27:50] 166 | pub use self::once::{Once, OnceState, ONCE_INIT};
[00:27:50]     |
[00:27:50]     = note: `-D deprecated` implied by `-D warnings`
[00:27:50] 
[00:27:52] error: aborting due to previous error
