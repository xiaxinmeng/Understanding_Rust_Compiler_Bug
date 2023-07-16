plain
travis_time:end:05454486:start=1545187482086206592,finish=1545187541815966755,duration=59729760163
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:41] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:41] tidy error: /checkout/src/libcore/mem.rs:617: line longer than 100 chars
[00:03:43] some tidy checks failed
[00:03:43] 
[00:03:43] 
[00:03:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:43] 
[00:03:43] 
[00:03:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:43] Build completed unsuccessfully in 0:00:45
[00:03:43] Build completed unsuccessfully in 0:00:45
[00:03:43] Makefile:79: recipe for target 'tidy' failed
[00:03:43] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12dd683e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 02:49:33 UTC 2018
---
travis_time:end:023f3028:start=1545187773701690394,finish=1545187773706967821,duration=5277427
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:025e00de
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0123c684
travis_time:start:0123c684
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20dc2772
$ dmesg | grep -i kill
