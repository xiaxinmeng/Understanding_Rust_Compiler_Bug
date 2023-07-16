plain
travis_time:end:018a6d90:start=1559240831287361816,finish=1559240832562018655,duration=1274656839
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:tidy
tidy check
[00:12:28] * 574 error codes
[00:12:28] * highest error code: E0729
[00:12:28] tidy error: /checkout/src/libcore/num/mod.rs:471: malformed stability attribute: missing the `since` key
[00:12:29] tidy error: /checkout/src/libcore/num/mod.rs:2520: malformed stability attribute: missing the `since` key
[00:12:33] some tidy checks failed
[00:12:33] 
[00:12:33] 
[00:12:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:12:33] 
[00:12:33] 
[00:12:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:12:33] Build completed unsuccessfully in 0:01:11
[00:12:33] Build completed unsuccessfully in 0:01:11
[00:12:33] Makefile:67: recipe for target 'tidy' failed
[00:12:33] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17935ef8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 18:39:56 UTC 2019
