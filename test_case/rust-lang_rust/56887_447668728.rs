plain
travis_time:end:10ff2c21:start=1544987564767243025,finish=1544987565864410530,duration=1097167505
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:02:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:45] tidy error: /checkout/src/test/run-pass/structs-enums/enum-non-c-like-repr-int.rs:24: line longer than 100 chars
[00:02:45] tidy error: /checkout/src/test/run-pass/structs-enums/enum-non-c-like-repr-c-and-int.rs:24: line longer than 100 chars
[00:02:45] tidy error: /checkout/src/test/run-pass/structs-enums/enum-non-c-like-repr-c.rs:24: line longer than 100 chars
[00:02:46] 
[00:02:46] 
[00:02:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:46] 
[00:02:46] 
[00:02:46] some tidy checks failed
[00:02:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:46] Build completed unsuccessfully in 0:00:42
[00:02:46] Makefile:79: recipe for target 'tidy' failed
[00:02:46] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12002c22
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 16 19:15:41 UTC 2018
---
travis_time:end:30102b58:start=1544987742178921394,finish=1544987742183753548,duration=4832154
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19106a24
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30f5cc2e
travis_time:start:30f5cc2e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:055b81c8
$ dmesg | grep -i kill
