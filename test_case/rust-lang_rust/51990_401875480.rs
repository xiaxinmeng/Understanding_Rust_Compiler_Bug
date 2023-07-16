plain
travis_time:start:tidy
tidy check
[00:04:19] * 550 error codes
[00:04:19] * highest error code: E0709
[00:04:19] Expected a gate test for the feature 'const_fn_union'.
[00:04:19] Hint: create a failing test file named 'feature-gate-const_fn_union.rs'
[00:04:19]       in the 'ui' test suite, with its failures due to
[00:04:19]       missing usage of #![feature(const_fn_union)].
[00:04:19] Hint: If you already have such a test and don't want to rename it,
[00:04:19]       you can also add a // gate-test-const_fn_union line to the test file.
[00:04:19] Expected a gate test for the feature 'const_raw_ptr_to_usize_cast'.
[00:04:19] Hint: create a failing test file named 'feature-gate-const_raw_ptr_to_usize_cast.rs'
[00:04:19]       in the 'ui' test suite, with its failures due to
[00:04:19]       missing usage of #![feature(const_raw_ptr_to_usize_cast)].
[00:04:19] Hint: If you already have such a test and don't want to rename it,
[00:04:19]       you can also add a // gate-test-const_raw_ptr_to_usize_cast line to the test file.
[00:04:19] Expected a gate test for the feature 'const_raw_ptr_deref'.
[00:04:19] Hint: create a failing test file named 'feature-gate-const_raw_ptr_deref.rs'
[00:04:19]       in the 'ui' test suite, with its failures due to
[00:04:19]       missing usage of #![feature(const_raw_ptr_deref)].
[00:04:19] Hint: If you already have such a test and don't want to rename it,
[00:04:19]       you can also add a // gate-test-const_raw_ptr_deref line to the test file.
[00:04:19] tidy error: Found 3 features without a gate test.
[00:04:20] some tidy checks failed
[00:04:20] 
[00:04:20] 
[00:04:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:20] 
[00:04:20] 
[00:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:20] Build completed unsuccessfully in 0:01:28
[00:04:20] Build completed unsuccessfully in 0:01:28
[00:04:20] Makefile:79: recipe for target 'tidy' failed
[00:04:20] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0122b1a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0795eec0:start=1530551679391518823,finish=1530551679398296832,duration=6778009
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02ae2a70
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025cdf00
$ dmesg | grep -i kill
