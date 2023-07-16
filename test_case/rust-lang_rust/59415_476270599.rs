plain
travis_time:end:1a0c7ac8:start=1553529000808050088,finish=1553530434909110763,duration=1434101060675
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:42] tidy error: /checkout/src/librustc_typeck/check/mod.rs:3232: line longer than 100 chars
[00:03:43] some tidy checks failed
[00:03:43] 
[00:03:43] 
[00:03:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:43] 
[00:03:43] 
[00:03:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:43] Build completed unsuccessfully in 0:00:44
[00:03:43] Build completed unsuccessfully in 0:00:44
[00:03:43] make: *** [tidy] Error 1
[00:03:43] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a61a792
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 16:17:48 UTC 2019
