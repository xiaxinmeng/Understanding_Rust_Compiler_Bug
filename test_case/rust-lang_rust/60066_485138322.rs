plain
travis_time:end:07c1ff0d:start=1555774337768260839,finish=1555774427672168973,duration=89903908134
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:25:07]    Compiling crossbeam-deque v0.2.0
[00:25:08] error: unnecessary `unsafe` block
[00:25:08]    --> src/libserialize/serialize.rs:839:16
[00:25:08]     |
[00:25:08] 839 |                unsafe { intrinsics::type_name::<S>() },
[00:25:08]     |
[00:25:08]     = note: `-D unused-unsafe` implied by `-D warnings`
[00:25:08] 
[00:25:08] error: unnecessary `unsafe` block
[00:25:08] error: unnecessary `unsafe` block
[00:25:08]    --> src/libserialize/serialize.rs:841:16
[00:25:08]     |
[00:25:08] 841 |                unsafe { intrinsics::type_name::<T>() },
[00:25:08] 
[00:25:08] error: aborting due to 2 previous errors
[00:25:08] 
[00:25:08] error: Could not compile `serialize`.
