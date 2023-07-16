plain
travis_time:end:06a32d30:start=1542810693329239542,finish=1542810694485834881,duration=1156595339
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:04:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:30] tidy error: /checkout/src/libcore/mem.rs:600: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/libcore/mem.rs:1082: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/libcore/mem.rs:1096: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/libcore/mem.rs:1107: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/libcore/mem.rs:1118: line longer than 100 chars
[00:04:31] some tidy checks failed
[00:04:31] 
[00:04:31] 
[00:04:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:31] 
[00:04:31] 
[00:04:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:31] Build completed unsuccessfully in 0:00:56
[00:04:31] Build completed unsuccessfully in 0:00:56
[00:04:31] Makefile:79: recipe for target 'tidy' failed
[00:04:31] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1df4c0cc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 14:36:15 UTC 2018
---
travis_time:end:12e91408:start=1542810975607506583,finish=1542810975612575847,duration=5069264
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:007812f9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e10c4b4
travis_time:start:0e10c4b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12492ae0
$ dmesg | grep -i kill
