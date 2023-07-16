plain
travis_time:end:11e078a8:start=1543165258662792543,finish=1543165260971378223,duration=2308585680
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
[00:03:01] * 569 error codes
[00:03:01] * highest error code: E9999
[00:03:01] tidy error: Found 1 features without a gate test.
[00:03:01] Expected a gate test for the feature 'optimize_attribute'.
[00:03:01] Hint: create a failing test file named 'feature-gate-optimize_attribute.rs'
[00:03:01]       in the 'ui' test suite, with its failures due to
[00:03:01]       missing usage of #![feature(optimize_attribute)].
[00:03:01] Hint: If you already have such a test and don't want to rename it,
[00:03:01]       you can also add a // gate-test-optimize_attribute line to the test file.
[00:03:02] some tidy checks failed
[00:03:02] 
[00:03:02] 
[00:03:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:02] 
[00:03:02] 
[00:03:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:02] Build completed unsuccessfully in 0:00:56
[00:03:02] Build completed unsuccessfully in 0:00:56
[00:03:02] Makefile:79: recipe for target 'tidy' failed
[00:03:02] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13bc2fb6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 25 17:04:12 UTC 2018
---
travis_time:end:143006b0:start=1543165452898384025,finish=1543165452903479782,duration=5095757
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08c2bb24
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b2352f1
travis_time:start:0b2352f1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:126bd930
$ dmesg | grep -i kill
