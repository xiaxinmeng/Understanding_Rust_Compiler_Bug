plain
travis_time:end:04ec8e44:start=1545222668182686871,finish=1545222768251413858,duration=100068726987
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:01] tidy error: /checkout/src/libstd/sys/sgx/rwlock.rs:228: trailing whitespace
[00:03:02] some tidy checks failed
[00:03:02] 
[00:03:02] 
[00:03:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:02] 
[00:03:02] 
[00:03:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:02] Build completed unsuccessfully in 0:00:43
[00:03:02] Build completed unsuccessfully in 0:00:43
[00:03:02] Makefile:79: recipe for target 'tidy' failed
[00:03:02] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bc56118
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 12:35:59 UTC 2018
---
travis_time:end:17350ff4:start=1545222959795432139,finish=1545222959799395270,duration=3963131
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06f78d38
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:193df331
travis_time:start:193df331
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06a4dce4
$ dmesg | grep -i kill
