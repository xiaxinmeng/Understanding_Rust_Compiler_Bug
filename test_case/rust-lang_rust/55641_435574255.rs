plain
travis_time:end:009315a3:start=1541237044741724763,finish=1541237046948850037,duration=2207125274
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:tidy
tidy check
[00:03:49] * 568 error codes
[00:03:49] * highest error code: E9999
[00:03:50] tidy error: Found 1 features without a gate test.
[00:03:50] Expected a gate test for the feature 'optimize_attribute'.
[00:03:50] Hint: create a failing test file named 'feature-gate-optimize_attribute.rs'
[00:03:50]       in the 'ui' test suite, with its failures due to
[00:03:50]       missing usage of #![feature(optimize_attribute)].
[00:03:50] Hint: If you already have such a test and don't want to rename it,
[00:03:50]       you can also add a // gate-test-optimize_attribute line to the test file.
[00:03:50] some tidy checks failed
[00:03:50] 
[00:03:50] 
[00:03:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:50] 
[00:03:50] 
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:50] Build completed unsuccessfully in 0:00:47
[00:03:50] Build completed unsuccessfully in 0:00:47
[00:03:50] Makefile:79: recipe for target 'tidy' failed
[00:03:50] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04133410
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:29a33223:start=1541237288665578516,finish=1541237288669755760,duration=4177244
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d79e220
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0014bf32
travis_time:start:0014bf32
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ca39a44
$ dmesg | grep -i kill
