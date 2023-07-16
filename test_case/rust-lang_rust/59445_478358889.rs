plain
travis_time:end:38abfb86:start=1554050643712835566,finish=1554050644824230562,duration=1111394996
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:18] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:19] tidy error: /checkout/src/librustc/ty/query/mod.rs:153: line longer than 100 chars
[00:04:20] some tidy checks failed
[00:04:20] 
[00:04:20] 
[00:04:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:20] 
[00:04:20] 
[00:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:20] Build completed unsuccessfully in 0:00:48
[00:04:20] Build completed unsuccessfully in 0:00:48
[00:04:20] Makefile:67: recipe for target 'tidy' failed
[00:04:20] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2dbb8df4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 31 16:48:37 UTC 2019
---
travis_time:end:3a6187ae:start=1554050918706529081,finish=1554050918712403628,duration=5874547
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0891aaac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06a69c84
travis_time:start:06a69c84
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b45b032
$ dmesg | grep -i kill
