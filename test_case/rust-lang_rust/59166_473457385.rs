plain
travis_time:end:014845bf:start=1552686990128486521,finish=1552687065743304101,duration=75614817580
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:38] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:39] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:899: TODO is deprecated; use FIXME
[00:03:39] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:911: line longer than 100 chars
[00:03:39] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:912: line longer than 100 chars
[00:03:39] tidy error: /checkout/src/librustc_resolve/lib.rs:4393: line longer than 100 chars
[00:03:39] tidy error: /checkout/src/librustc_resolve/lib.rs:4412: line longer than 100 chars
[00:03:40] some tidy checks failed
[00:03:40] 
[00:03:40] 
[00:03:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:40] 
[00:03:40] 
[00:03:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:40] Build completed unsuccessfully in 0:00:48
[00:03:40] Build completed unsuccessfully in 0:00:48
[00:03:40] Makefile:67: recipe for target 'tidy' failed
[00:03:40] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02fa50c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 15 22:01:34 UTC 2019
---
travis_time:end:12cf1686:start=1552687295649469690,finish=1552687295654195313,duration=4725623
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:255d56db
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b552b78
travis_time:start:1b552b78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cbcb5bf
$ dmesg | grep -i kill
