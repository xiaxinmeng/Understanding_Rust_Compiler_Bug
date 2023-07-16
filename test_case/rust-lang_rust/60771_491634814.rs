plain
travis_time:end:0d8fe389:start=1557699909817230431,finish=1557699910600900221,duration=783669790
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:21]     |
[00:05:21] 138 | unsafe impl Alloc for System {
[00:05:21]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Err` in implementation
[00:05:21]     |
[00:05:21]     = note: `Err` from trait: `type Err;`
[00:05:21] error: aborting due to previous error
[00:05:21] 
[00:05:21] For more information about this error, try `rustc --explain E0046`.
[00:05:21] error: Could not compile `std`.
