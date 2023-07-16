plain
travis_time:end:10fe8c64:start=1541894391748079033,finish=1541894449469933326,duration=57721854293
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:45] tidy error: /checkout/src/librustc_typeck/collect.rs:1331: line longer than 100 chars
[00:03:46] some tidy checks failed
[00:03:46] 
[00:03:46] 
[00:03:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:46] 
[00:03:46] 
[00:03:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:46] Build completed unsuccessfully in 0:00:48
[00:03:46] Build completed unsuccessfully in 0:00:48
[00:03:46] make: *** [tidy] Error 1
[00:03:46] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fac5a00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 11 00:04:47 UTC 2018
---
travis_time:end:01f4ca30:start=1541894688300458401,finish=1541894688305636127,duration=5177726
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:130c2fe4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06c4e43e
travis_time:start:06c4e43e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18915922
$ dmesg | grep -i kill
