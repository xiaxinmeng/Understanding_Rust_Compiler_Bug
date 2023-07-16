plain
travis_time:end:199fc618:start=1541730792869377648,finish=1541730848583495443,duration=55714117795
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:07:28]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:01]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:01]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:13:46]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:48] error: unused import: `DefIdTree`
[00:13:48]   --> librustc_typeck/check/coercion.rs:71:23
[00:13:48]    |
[00:13:48] 71 | use rustc::ty::{self, DefIdTree, TypeAndMut, Ty, ClosureSubsts};
[00:13:48]    |
[00:13:48]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:48] 
[00:13:53] error: aborting due to previous error
