plain
travis_time:end:09150242:start=1550042739175615429,finish=1550042741680382993,duration=2504767564
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
[00:03:35] * 565 error codes
[00:03:35] * highest error code: E0722
[00:03:36] * 250 features
[00:03:36] tidy error: /checkout/src/libstd/thread/tests.rs:1: platform-specific cfg: cfg(not(target_os = "emscripten"))
[00:03:36] tidy error: /checkout/src/libstd/thread/tests.rs:251: platform-specific cfg: cfg(not(target_os = "emscripten"))
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:46
[00:03:37] Build completed unsuccessfully in 0:00:46
[00:03:37] Makefile:68: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aff0754
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 07:29:29 UTC 2019
---
travis_time:end:086cb752:start=1550042970347779856,finish=1550042970352792158,duration=5012302
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:118c4038
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:078ec5f4
travis_time:start:078ec5f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00e50a28
$ dmesg | grep -i kill
