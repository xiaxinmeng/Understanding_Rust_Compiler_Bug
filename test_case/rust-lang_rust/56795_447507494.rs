plain
travis_time:end:0db5ca40:start=1544827260137653807,finish=1544827315500349307,duration=55362695500
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:35:50]    Compiling rand_pcg v0.1.1
[00:35:50]    Compiling rand v0.6.1
[00:35:51]    Compiling parking_lot_core v0.3.0
[00:35:56]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:35:56] error[E0463]: can't find crate for `serde_derive` which `rustc` depends on
[00:35:56]    |
[00:35:56] 34 | extern crate rustc;
[00:35:56]    | ^^^^^^^^^^^^^^^^^^^ can't find crate
[00:35:56] 
---
[00:35:56] 
[00:35:56] 
[00:35:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:35:56] Build completed unsuccessfully in 0:32:55
[00:35:56] Makefile:28: recipe for target 'all' failed
[00:35:56] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0005b973
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 14 23:18:00 UTC 2018
