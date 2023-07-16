plain
travis_time:end:0f045a1a:start=1560928500740750172,finish=1560928501881683377,duration=1140933205
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:45]    Compiling cc v1.0.35
[00:04:45]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:45]    Compiling libc v0.2.54
[00:04:45]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:45] error[E0584]: file for module `ptr` found at both ptr.rs and ptr/mod.rs
[00:04:45]     |
[00:04:45] 176 | pub mod ptr;
[00:04:45]     |         ^^^
[00:04:45]     |
[00:04:45]     |
[00:04:45]     = help: delete or rename one of them to remove the ambiguity
[00:04:45] error: aborting due to previous error
[00:04:45] 
[00:04:45] error: Could not compile `core`.
[00:04:45] warning: build failed, waiting for other jobs to finish...
