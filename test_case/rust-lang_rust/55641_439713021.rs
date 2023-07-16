plain
travis_time:end:025316be:start=1542563866528197442,finish=1542563867660478479,duration=1132281037
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
[00:04:09] * 569 error codes
[00:04:09] * highest error code: E9999
[00:04:09] tidy error: Found 1 features without a gate test.
[00:04:09] Expected a gate test for the feature 'optimize_attribute'.
[00:04:09] Hint: create a failing test file named 'feature-gate-optimize_attribute.rs'
[00:04:09]       in the 'ui' test suite, with its failures due to
[00:04:09]       missing usage of #![feature(optimize_attribute)].
[00:04:09] Hint: If you already have such a test and don't want to rename it,
[00:04:09]       you can also add a // gate-test-optimize_attribute line to the test file.
[00:04:10] some tidy checks failed
[00:04:10] 
[00:04:10] 
[00:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:10] 
[00:04:10] 
[00:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:10] Build completed unsuccessfully in 0:00:43
[00:04:10] Build completed unsuccessfully in 0:00:43
[00:04:10] Makefile:79: recipe for target 'tidy' failed
[00:04:10] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04e66bd9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 18 18:02:06 UTC 2018
---
travis_time:end:030b2d33:start=1542564127238716706,finish=1542564127243395009,duration=4678303
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2fe813f3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02f277c2
travis_time:start:02f277c2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06d9b8cc
$ dmesg | grep -i kill
