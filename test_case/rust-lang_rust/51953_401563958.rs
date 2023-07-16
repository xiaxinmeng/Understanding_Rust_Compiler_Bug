plain
travis_time:start:tidy
tidy check
[00:04:58] * 554 error codes
[00:04:58] * highest error code: E0709
[00:04:59] Expected a gate test for the feature 'cfg_target_has_atomic_cas'.
[00:04:59] Hint: create a failing test file named 'feature-gate-cfg_target_has_atomic_cas.rs'
[00:04:59]       in the 'ui' test suite, with its failures due to
[00:04:59]       missing usage of #![feature(cfg_target_has_atomic_cas)].
[00:04:59] Hint: If you already have such a test and don't want to rename it,
[00:04:59]       you can also add a // gate-test-cfg_target_has_atomic_cas line to the test file.
[00:04:59] tidy error: Found 1 features without a gate test.
[00:05:00] some tidy checks failed
[00:05:00] 
[00:05:00] 
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:00] 
[00:05:00] 
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:00] Build completed unsuccessfully in 0:01:44
[00:05:00] Build completed unsuccessfully in 0:01:44
[00:05:00] Makefile:79: recipe for target 'tidy' failed
[00:05:00] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bf598a7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:09529b6e:start=1530389450177286860,finish=1530389450184758329,duration=7471469
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13d7d22e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f89848
$ dmesg | grep -i kill
