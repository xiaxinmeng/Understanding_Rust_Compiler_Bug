plain
travis_time:end:0db28630:start=1543530490568440147,finish=1543530491588322461,duration=1019882314
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:02:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:00] tidy error: /checkout/src/librustc/lint/mod.rs:117: line longer than 100 chars
[00:03:01] some tidy checks failed
[00:03:01] 
[00:03:01] 
[00:03:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:01] 
[00:03:01] 
[00:03:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:01] Build completed unsuccessfully in 0:00:56
[00:03:01] Build completed unsuccessfully in 0:00:56
[00:03:01] Makefile:79: recipe for target 'tidy' failed
[00:03:01] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2457b080
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 29 22:31:22 UTC 2018
---
travis_time:end:21b49f30:start=1543530682927527100,finish=1543530682932148535,duration=4621435
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2554dcdf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:039f17d2
travis_time:start:039f17d2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22356153
$ dmesg | grep -i kill
