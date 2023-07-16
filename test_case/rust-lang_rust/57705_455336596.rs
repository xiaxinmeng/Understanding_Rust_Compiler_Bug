plain
travis_time:end:0e49e2b6:start=1547758001164401753,finish=1547758002448970093,duration=1284568340
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:54] tidy error: /checkout/src/librustc_target/spec/powerpc64_unknown_freebsd.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:56] some tidy checks failed
[00:03:56] 
[00:03:56] 
[00:03:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:56] 
[00:03:56] 
[00:03:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:56] Build completed unsuccessfully in 0:00:46
[00:03:56] Build completed unsuccessfully in 0:00:46
[00:03:56] Makefile:69: recipe for target 'tidy' failed
[00:03:56] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0930362a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 20:50:49 UTC 2019
---
travis_time:end:0471a716:start=1547758250046035624,finish=1547758250051858004,duration=5822380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dfabb46
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2cfd64a8
travis_time:start:2cfd64a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08325c10
$ dmesg | grep -i kill
