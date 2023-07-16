plain
travis_time:end:08a98434:start=1543285648656221918,finish=1543285649799742662,duration=1143520744
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:36]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:38] error: unused import: `Place`
[00:13:38]   --> src/librustc_mir/borrow_check/borrow_set.rs:19:39
[00:13:38]    |
[00:13:38] 19 | use rustc::mir::{self, Location, Mir, Place, Local};
[00:13:38]    |
[00:13:38]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:38] 
[00:13:52] error: aborting due to previous error
