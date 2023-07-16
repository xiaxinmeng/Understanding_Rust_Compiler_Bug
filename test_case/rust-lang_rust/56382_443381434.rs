plain
travis_time:end:2426b70d:start=1543624268640464435,finish=1543624271025923317,duration=2385458882
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:04] tidy error: /checkout/src/librustc_traits/dropck_outlives.rs:33: line longer than 100 chars
[00:03:04] tidy error: /checkout/src/librustc/ty/mod.rs:138: trailing whitespace
[00:03:04] tidy error: /checkout/src/librustc/ty/mod.rs:151: trailing whitespace
[00:03:04] tidy error: /checkout/src/librustc/ty/codec.rs:217: line longer than 100 chars
[00:03:05] some tidy checks failed
[00:03:05] 
[00:03:05] 
[00:03:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:05] 
[00:03:05] 
[00:03:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:05] Build completed unsuccessfully in 0:00:58
[00:03:05] Build completed unsuccessfully in 0:00:58
[00:03:05] make: *** [tidy] Error 1
[00:03:05] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1377b693
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  1 00:34:26 UTC 2018
---
travis_time:end:1173c25e:start=1543624466596980898,finish=1543624466602742226,duration=5761328
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:060d3788
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13659312
travis_time:start:13659312
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:004ca860
$ dmesg | grep -i kill
