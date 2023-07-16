plain
travis_time:end:013d8ea4:start=1542363482690847273,finish=1542363541483415501,duration=58792568228
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:44] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:44] tidy error: /checkout/src/librustc_mir/interpret/memory.rs:671: line longer than 100 chars
[00:03:45] some tidy checks failed
[00:03:45] 
[00:03:45] 
[00:03:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:45] 
[00:03:45] 
[00:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:45] Build completed unsuccessfully in 0:00:46
[00:03:45] Build completed unsuccessfully in 0:00:46
[00:03:45] Makefile:79: recipe for target 'tidy' failed
[00:03:45] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:084d1cac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 16 10:22:57 UTC 2018
---
travis_time:end:0db79445:start=1542363778173239337,finish=1542363778177877849,duration=4638512
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00570bbc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:191a90b4
travis_time:start:191a90b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fe39998
$ dmesg | grep -i kill
