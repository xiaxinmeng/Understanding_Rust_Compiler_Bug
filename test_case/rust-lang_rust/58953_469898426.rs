plain
travis_time:end:00f55755:start=1551827454640017285,finish=1551827455546996046,duration=906978761
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:52]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:59] error: use has missing stability attribute
[00:04:59]   --> src/libstd/sys/unix/ext/ffi.rs:37:1
[00:04:59]    |
[00:04:59] 37 | pub use crate::sys_common::ffi_bytes::*;
[00:04:59] 
travis_time:end:12e1d180:start=1551827466555934200,finish=1551827766510703537,duration=299954769337
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03134afc
