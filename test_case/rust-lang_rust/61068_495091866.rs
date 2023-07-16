plain
travis_time:end:03269e96:start=1558594131965403944,finish=1558594220356508478,duration=88391104534
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:28]    Compiling test v0.0.0 (/checkout/src/libtest)
[00:05:28] error[E0308]: mismatched types
[00:05:28]     --> src/libtest/lib.rs:1546:26
[00:05:28]      |
[00:05:28] 1546 |         _ => TrFailedMsg("test did not panic as expected"),
[00:05:28]      |                          |
[00:05:28]      |                          expected struct `std::string::String`, found reference
[00:05:28]      |                          expected struct `std::string::String`, found reference
[00:05:28]      |                          help: try using a conversion method: `"test did not panic as expected".to_string()`
[00:05:28]      = note: expected type `std::string::String`
[00:05:28]                 found type `&'static str`
[00:05:28] 
[00:05:29] error: aborting due to previous error
