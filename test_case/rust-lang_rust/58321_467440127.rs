plain
travis_time:end:2c0dfee8:start=1551187780252090094,finish=1551187782542408351,duration=2290318257
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:24] tidy error: /checkout/src/librustc/traits/object_safety.rs:562: line longer than 100 chars
[00:04:26] some tidy checks failed
[00:04:26] 
[00:04:26] 
[00:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:26] 
[00:04:26] 
[00:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:26] Build completed unsuccessfully in 0:00:49
[00:04:26] Build completed unsuccessfully in 0:00:49
[00:04:26] Makefile:68: recipe for target 'tidy' failed
[00:04:26] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26bedad0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 13:34:19 UTC 2019
---
travis_time:end:01a33a7d:start=1551188060528664046,finish=1551188060533746558,duration=5082512
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0af9b874
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1df0ff83
travis_time:start:1df0ff83
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a522b78
$ dmesg | grep -i kill
