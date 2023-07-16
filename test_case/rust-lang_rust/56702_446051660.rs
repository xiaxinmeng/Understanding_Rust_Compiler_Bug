plain
travis_time:end:26dc19b0:start=1544495328728538054,finish=1544495329794686848,duration=1066148794
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:12] tidy error: /checkout/src/librustc/util/profiling.rs:65: line longer than 100 chars
[00:03:12] tidy error: /checkout/src/librustc/util/profiling.rs:67: line longer than 100 chars
[00:03:13] some tidy checks failed
[00:03:13] 
[00:03:13] 
[00:03:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:13] 
[00:03:13] 
[00:03:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:13] Build completed unsuccessfully in 0:00:56
[00:03:13] Build completed unsuccessfully in 0:00:56
[00:03:13] Makefile:79: recipe for target 'tidy' failed
[00:03:13] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2fd12fce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 11 02:32:12 UTC 2018
---
travis_time:end:0e6bfad0:start=1544495532778285800,finish=1544495532783801955,duration=5516155
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2794e5e4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0236423a
travis_time:start:0236423a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1802c0f4
$ dmesg | grep -i kill
