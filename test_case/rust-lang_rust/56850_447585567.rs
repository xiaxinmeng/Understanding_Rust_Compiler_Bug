plain
travis_time:end:0186fa5c:start=1544895218629916967,finish=1544895220797194802,duration=2167277835
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:11:30]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:11:32] error[E0603]: enum `AdtKind` is private
[00:11:32]   --> src/librustc_typeck/check/mod.rs:98:24
[00:11:32]    |
[00:11:32] 98 | use rustc::hir::{self, AdtKind, GenericArg, ItemKind, Node, PatKind};
[00:11:32] 
[00:11:38] error: aborting due to previous error
[00:11:38] 
[00:11:38] For more information about this error, try `rustc --explain E0603`.
