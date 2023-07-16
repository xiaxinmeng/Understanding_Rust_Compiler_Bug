plain
travis_time:end:25b5bbb8:start=1559187520651196332,finish=1559187521388138197,duration=736941865
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:36]    Compiling backtrace v0.3.25
[00:04:37] error[E0425]: cannot find value `ptr` in this scope
[00:04:37]     --> src/libcore/str/mod.rs:1420:17
[00:04:37]      |
[00:04:37] 1420 |     let align = ptr.align_offset(usize_bytes);
[00:04:37] 
[00:04:40]    Compiling compiler_builtins v0.1.15
[00:04:40]    Compiling cmake v0.1.38
[00:04:40]    Compiling backtrace-sys v0.1.27
