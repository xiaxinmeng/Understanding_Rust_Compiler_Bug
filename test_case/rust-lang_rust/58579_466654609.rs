plain
travis_time:end:078e3928:start=1550927821310640266,finish=1550927822688746675,duration=1378106409
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:57:38]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:57:41] error[E0432]: unresolved import `super::acle`
[00:57:41]   --> src/libcore/../stdsimd/crates/core_arch/src/aarch64/mod.rs:21:16
[00:57:41]    |
[00:57:41] 21 | pub use super::acle::*;
[00:57:41]    |                ^^^^ could not find `acle` in `super`
[00:57:41] error[E0432]: unresolved import `crate::core_arch::acle`
[00:57:41]   --> src/libcore/../stdsimd/crates/core_arch/src/arm/mod.rs:37:27
[00:57:41]    |
[00:57:41]    |
[00:57:41] 37 | pub use crate::core_arch::acle::*;
[00:57:41]    |                           ^^^^ could not find `acle` in `core_arch`
Sat, 23 Feb 2019 14:15:04 GMT
travis_time:end:016f37e0:start=1550931303749143003,finish=1550931303913504790,duration=164361787
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
