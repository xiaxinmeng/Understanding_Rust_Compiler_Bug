plain
travis_time:end:00121e33:start=1558816909088942334,finish=1558816997726707070,duration=88637764736
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:50]    Compiling hashbrown v0.3.0
[00:04:53] error[E0202]: associated types are not yet supported in inherent impls (see #8995)
[00:04:53]    --> src/libstd/ffi/os_str.rs:975:5
[00:04:53]     |
[00:04:53] 975 |     type Output = OsString;
[00:04:53] 
[00:04:53] error: aborting due to previous error
[00:04:53] 
[00:04:53] For more information about this error, try `rustc --explain E0202`.
