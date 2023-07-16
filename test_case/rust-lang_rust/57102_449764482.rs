plain
travis_time:end:2b13274e:start=1545676453477515398,finish=1545676455687540886,duration=2210025488
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:23] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:23] tidy error: /checkout/src/test/ui/issues/issue-45157.rs:38: line longer than 100 chars
[00:03:24] some tidy checks failed
[00:03:24] 
[00:03:24] 
[00:03:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:24] 
[00:03:24] 
[00:03:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:24] Build completed unsuccessfully in 0:00:43
[00:03:24] Build completed unsuccessfully in 0:00:43
[00:03:24] Makefile:79: recipe for target 'tidy' failed
[00:03:24] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24f077a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 24 18:37:48 UTC 2018
---
travis_time:end:0b380350:start=1545676669062185744,finish=1545676669066475132,duration=4289388
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:118bb02a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1906a6e9
travis_time:start:1906a6e9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1712a987
$ dmesg | grep -i kill
