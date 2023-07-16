plain
travis_time:end:00aeda44:start=1543854978675823660,finish=1543854980011010775,duration=1335187115
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:47] tidy error: /checkout/src/libcore/result.rs:946: trailing whitespace
[00:03:48] some tidy checks failed
[00:03:48] 
[00:03:48] 
[00:03:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:48] 
[00:03:48] 
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] Build completed unsuccessfully in 0:00:55
[00:03:48] Build completed unsuccessfully in 0:00:55
[00:03:48] make: *** [tidy] Error 1
[00:03:48] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0750afac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 16:40:17 UTC 2018
