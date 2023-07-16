plain
travis_time:end:09be4e26:start=1542820478157273208,finish=1542820479190801499,duration=1033528291
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
[00:04:31] Build completed unsuccessfully in 0:00:52
[00:04:31] Build completed unsuccessfully in 0:00:52
[00:04:31] make: *** [tidy] Error 1
[00:04:31] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0863837c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 17:19:18 UTC 2018
---
travis_time:end:064723c2:start=1542820759259650560,finish=1542820759263592431,duration=3941871
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18413353
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18aee283
travis_time:start:18aee283
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:007d8596
$ dmesg | grep -i kill
