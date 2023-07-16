plain
travis_time:end:105fca08:start=1552584032094632030,finish=1552584034544468315,duration=2449836285
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:52]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:41]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:59]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:59]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:02] error: unused import: `DefIdTree`
[00:14:02]   --> src/librustc_mir/borrow_check/mod.rs:15:23
[00:14:02]    |
[00:14:02] 15 | use rustc::ty::{self, DefIdTree, TyCtxt};
[00:14:02]    |
[00:14:02]    = note: `-D unused-imports` implied by `-D warnings`
[00:14:02] 
[00:14:14] error: aborting due to previous error
---
156800 ./obj/build/bootstrap/debug/incremental
156416 ./src/llvm-project/clang
150472 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
141936 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex
141932 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex/s-facfbzx4za-101gpta-3clbg54leqh9a
108532 ./src/llvm-project/lldb
97584 ./src/llvm-project/clang/test
95632 ./.git
93100 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
