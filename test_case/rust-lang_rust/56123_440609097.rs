plain
travis_time:end:01036c3a:start=1542794324618965122,finish=1542794381413467134,duration=56794502012
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
[00:03:52] * 568 error codes
[00:03:52] * highest error code: E0721
[00:03:52] tidy error: libsyntax/feature_gate.rs:236: no tracking issue for feature unleash_the_miri_inside_of_you
[00:03:52] Expected a gate test for the feature 'unleash_the_miri_inside_of_you'.
[00:03:52] Hint: create a failing test file named 'feature-gate-unleash_the_miri_inside_of_you.rs'
[00:03:52]       in the 'ui' test suite, with its failures due to
[00:03:52]       missing usage of #![feature(unleash_the_miri_inside_of_you)].
[00:03:52] Hint: If you already have such a test and don't want to rename it,
[00:03:52]       you can also add a // gate-test-unleash_the_miri_inside_of_you line to the test file.
[00:03:52] tidy error: Found 1 features without a gate test.
[00:03:52] tidy error: libsyntax/feature_gate.rs:236: no tracking issue for feature unleash_the_miri_inside_of_you
[00:03:53] some tidy checks failed
[00:03:53] 
[00:03:53] 
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:53] 
[00:03:53] 
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:53] Build completed unsuccessfully in 0:00:56
[00:03:53] Build completed unsuccessfully in 0:00:56
[00:03:53] Makefile:79: recipe for target 'tidy' failed
[00:03:53] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ca495c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 10:03:43 UTC 2018
---
travis_time:end:1f56b742:start=1542794624411797059,finish=1542794624417458025,duration=5660966
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b6d6dfc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:259e7aa9
travis_time:start:259e7aa9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ace1e30
$ dmesg | grep -i kill
