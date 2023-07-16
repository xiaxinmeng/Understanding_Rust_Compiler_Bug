plain
travis_time:end:14b1805d:start=1544232075486772405,finish=1544232076507649915,duration=1020877510
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:08] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:08] tidy error: /checkout/src/test/run-pass/structs-enums/enum-non-c-like-repr-int.rs:24: line longer than 100 chars
[00:03:09] tidy error: /checkout/src/test/run-pass/structs-enums/enum-non-c-like-repr-c-and-int.rs:24: line longer than 100 chars
[00:03:09] tidy error: /checkout/src/test/run-pass/structs-enums/enum-non-c-like-repr-c.rs:24: line longer than 100 chars
[00:03:10] some tidy checks failed
[00:03:10] 
[00:03:10] 
[00:03:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:10] 
[00:03:10] 
[00:03:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:10] Build completed unsuccessfully in 0:00:55
[00:03:10] Build completed unsuccessfully in 0:00:55
[00:03:10] make: *** [tidy] Error 1
[00:03:10] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:081eede2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 01:24:35 UTC 2018
---
travis_time:end:346fa3e8:start=1544232275942384796,finish=1544232275947218126,duration=4833330
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09a98d56
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0eb00717
travis_time:start:0eb00717
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13c155a8
$ dmesg | grep -i kill
