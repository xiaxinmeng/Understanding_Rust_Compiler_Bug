plain
travis_time:end:0164d40f:start=1555555460697551711,finish=1555555461469736846,duration=772185135
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:44]     |
[00:04:44] 468 |     intrinsics::type_name::<T>()
[00:04:44]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[00:04:44]     |
[00:04:44]     = note: consult the function's documentation for information on how to avoid undefined behavior
[00:04:44] error: aborting due to previous error
[00:04:44] 
[00:04:44] For more information about this error, try `rustc --explain E0133`.
[00:04:44] error: Could not compile `core`.
