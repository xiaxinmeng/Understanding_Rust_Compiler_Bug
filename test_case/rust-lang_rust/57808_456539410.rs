plain
travis_time:end:09982f08:start=1548184560769430771,finish=1548184649899393436,duration=89129962665
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
tidy check
[00:03:34] * 564 error codes
[00:03:34] * highest error code: E0721
[00:03:34] * 246 features
[00:03:34] tidy error: /checkout/src/libstd/tests/run-time-detect.rs:4: platform-specific cfg: cfg(all(target_arch = "arm", any(target_os = "linux", target_os = "android")))
[00:03:34] tidy error: /checkout/src/libstd/tests/run-time-detect.rs:11: platform-specific cfg: cfg(all(
[00:03:34]     target_arch = "aarch64",
[00:03:34]     any(target_os = "linux", target_os = "android")
[00:03:34] ))
[00:03:34] tidy error: /checkout/src/libstd/tests/run-time-detect.rs:30: platform-specific cfg: cfg(all(target_arch = "powerpc", target_os = "linux"))
[00:03:34] tidy error: /checkout/src/libstd/tests/run-time-detect.rs:38: platform-specific cfg: cfg(all(target_arch = "powerpc64", target_os = "linux"))
[00:03:35] some tidy checks failed
[00:03:35] 
[00:03:35] 
[00:03:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:35] 
[00:03:35] 
[00:03:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:35] Build completed unsuccessfully in 0:00:45
[00:03:35] Build completed unsuccessfully in 0:00:45
[00:03:35] Makefile:69: recipe for target 'tidy' failed
[00:03:35] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0861464a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 22 19:21:15 UTC 2019
---
travis_time:end:05983b5a:start=1548184875753804193,finish=1548184875758129648,duration=4325455
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:070451df
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0613d602
travis_time:start:0613d602
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17c97226
$ dmesg | grep -i kill
