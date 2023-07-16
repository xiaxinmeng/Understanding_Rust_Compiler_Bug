plain
travis_time:end:002eb60c:start=1549187258651449357,finish=1549187347673838601,duration=89022389244
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:19] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:19] tidy error: /checkout/src/librustc_mir/build/matches/test.rs:436: TODO is deprecated; use FIXME
[00:03:19] tidy error: /checkout/src/librustc_mir/build/matches/test.rs:671: TODO is deprecated; use FIXME
[00:03:19] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:957: line longer than 100 chars
[00:03:19] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:1278: TODO is deprecated; use FIXME
[00:03:21] some tidy checks failed
[00:03:21] 
[00:03:21] 
[00:03:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:21] 
[00:03:21] 
[00:03:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:21] Build completed unsuccessfully in 0:00:44
[00:03:21] Build completed unsuccessfully in 0:00:44
[00:03:21] make: *** [tidy] Error 1
[00:03:21] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b12b611
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 09:52:38 UTC 2019
---
travis_time:end:02a1ea96:start=1549187559666826861,finish=1549187559671302697,duration=4475836
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:011f6e08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0217c740
travis_time:start:0217c740
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07321aca
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[    0.939064] misc rfkill: hash matches
travis_fold:end:after_failure.6

Done. Your build exited with 1.
