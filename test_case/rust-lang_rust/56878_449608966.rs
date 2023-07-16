plain
travis_time:end:16d958a8:start=1545529510915685481,finish=1545529511952489505,duration=1036804024
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:02:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:49] tidy error: /checkout/src/librustc_privacy/lib.rs:57: line longer than 100 chars
[00:02:49] tidy error: /checkout/src/librustc_privacy/lib.rs:128: line longer than 100 chars
[00:02:50] some tidy checks failed
[00:02:50] 
[00:02:50] 
[00:02:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:50] 
[00:02:50] 
[00:02:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:50] Build completed unsuccessfully in 0:00:43
[00:02:50] Build completed unsuccessfully in 0:00:43
[00:02:50] Makefile:79: recipe for target 'tidy' failed
[00:02:50] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26d15d35
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 23 01:48:11 UTC 2018
---
travis_time:end:07eacdf0:start=1545529691599781971,finish=1545529691604151826,duration=4369855
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0aa11c98
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dd2d23c
travis_time:start:0dd2d23c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11128fd7
$ dmesg | grep -i kill
