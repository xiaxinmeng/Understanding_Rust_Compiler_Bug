plain
travis_time:end:10c00696:start=1541895262261391670,finish=1541895315574570902,duration=53313179232
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:52] tidy error: /checkout/src/librustc_typeck/collect.rs:1331: line longer than 100 chars
[00:03:53] some tidy checks failed
[00:03:53] 
[00:03:53] 
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:53] 
[00:03:53] 
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:53] Build completed unsuccessfully in 0:00:47
[00:03:53] Build completed unsuccessfully in 0:00:47
[00:03:53] Makefile:79: recipe for target 'tidy' failed
[00:03:53] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05a9f150
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 11 00:19:18 UTC 2018
---
travis_time:end:072a31e8:start=1541895559309765190,finish=1541895559315182668,duration=5417478
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00632232
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b357130
travis_time:start:0b357130
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10cd7318
$ dmesg | grep -i kill
