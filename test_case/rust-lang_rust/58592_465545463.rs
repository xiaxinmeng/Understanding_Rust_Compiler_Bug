plain
travis_time:end:362df60e:start=1550663127255856788,finish=1550663129465543049,duration=2209686261
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:11] tidy error: /checkout/src/librustc/infer/mod.rs:968: line longer than 100 chars
[00:04:12] some tidy checks failed
[00:04:12] 
[00:04:12] 
[00:04:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:12] 
[00:04:12] 
[00:04:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:12] Build completed unsuccessfully in 0:00:45
[00:04:12] Build completed unsuccessfully in 0:00:45
[00:04:12] make: *** [tidy] Error 1
[00:04:12] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2cec4ad8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 11:49:53 UTC 2019
---
travis_time:end:15196653:start=1550663393681474415,finish=1550663393685718651,duration=4244236
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19e3adfa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1de0386f
travis_time:start:1de0386f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03da50bf
$ dmesg | grep -i kill
