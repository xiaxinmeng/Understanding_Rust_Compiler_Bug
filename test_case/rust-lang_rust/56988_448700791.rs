plain
travis_time:end:353f0f89:start=1545244467577054256,finish=1545244468694615649,duration=1117561393
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
[00:03:00] * 568 error codes
[00:03:00] * highest error code: E0721
[00:03:00] * 244 features
[00:03:00] tidy error: /checkout/src/libstd/time.rs:192: platform-specific cfg: cfg!(target_os = "macos")
[00:03:00] tidy error: /checkout/src/libstd/time.rs:193: platform-specific cfg: cfg!(target_os = "linux")
[00:03:00] tidy error: /checkout/src/libstd/time.rs:194: platform-specific cfg: cfg!(target_os = "linux")
[00:03:01] some tidy checks failed
[00:03:01] 
[00:03:01] 
[00:03:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:01] 
[00:03:01] 
[00:03:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:01] Build completed unsuccessfully in 0:00:45
[00:03:01] Build completed unsuccessfully in 0:00:45
[00:03:01] Makefile:79: recipe for target 'tidy' failed
[00:03:01] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:262a6f76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 18:37:39 UTC 2018
---
travis_time:end:0cee41d8:start=1545244659972411142,finish=1545244659977875228,duration=5464086
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ad9f916
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:116c0f86
travis_time:start:116c0f86
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09d86d7a
$ dmesg | grep -i kill
