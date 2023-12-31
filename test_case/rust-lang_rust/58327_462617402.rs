plain
travis_time:end:17217c8c:start=1549945738004084576,finish=1549946145855645591,duration=407851561015
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
[00:03:40] * 568 error codes
[00:03:40] * highest error code: E0726
[00:03:40] * 252 features
[00:03:40] tidy error: The Unstable Book has a 'language feature' section 'c_ffi_const' which doesn't correspond to an unstable language feature
[00:03:40] tidy error: The Unstable Book has a 'language feature' section 'c_ffi_pure' which doesn't correspond to an unstable language feature
[00:03:41] some tidy checks failed
[00:03:41] 
[00:03:41] 
[00:03:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:41] 
[00:03:41] 
[00:03:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:41] Build completed unsuccessfully in 0:00:48
[00:03:41] Build completed unsuccessfully in 0:00:48
[00:03:41] Makefile:68: recipe for target 'tidy' failed
[00:03:41] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:281d2980
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 04:39:36 UTC 2019
---
travis_time:end:2a07bc66:start=1549946377288241187,finish=1549946377293431369,duration=5190182
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bc5c900
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:34c7a400
travis_time:start:34c7a400
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22bf8647
$ dmesg | grep -i kill
