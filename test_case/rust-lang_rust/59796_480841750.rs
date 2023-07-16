plain
travis_time:end:36349119:start=1554730762461207273,finish=1554730763492765294,duration=1031558021
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:54] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:551: line longer than 100 chars
[00:03:54] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:559: line longer than 100 chars
[00:03:56] some tidy checks failed
[00:03:56] 
[00:03:56] 
[00:03:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:56] 
[00:03:56] 
[00:03:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:56] Build completed unsuccessfully in 0:00:43
[00:03:56] Build completed unsuccessfully in 0:00:43
[00:03:56] make: *** [tidy] Error 1
[00:03:56] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:096e6d34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  8 13:43:30 UTC 2019
---
travis_time:end:2ce30284:start=1554731011426920816,finish=1554731011431819407,duration=4898591
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b113d96
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ef84940
travis_time:start:0ef84940
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e649ef8
$ dmesg | grep -i kill
