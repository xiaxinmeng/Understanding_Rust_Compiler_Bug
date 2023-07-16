plain
travis_time:start:tidy
tidy check
[00:04:13] * 553 error codes
[00:04:13] * highest error code: E0710
[00:04:13] Expected a gate test for the feature 'macros_2'.
[00:04:13] Hint: create a failing test file named 'feature-gate-macros_2.rs'
[00:04:13]       in the 'ui' test suite, with its failures due to
[00:04:13]       missing usage of #![feature(macros_2)].
[00:04:13] Hint: If you already have such a test and don't want to rename it,
[00:04:13]       you can also add a // gate-test-macros_2 line to the test file.
[00:04:13] tidy error: Found 1 features without a gate test.
[00:04:14] some tidy checks failed
[00:04:14] 
[00:04:14] 
[00:04:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:14] 
[00:04:14] 
[00:04:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:14] Build completed unsuccessfully in 0:00:54
[00:04:14] Build completed unsuccessfully in 0:00:54
[00:04:14] Makefile:79: recipe for target 'tidy' failed
[00:04:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08b03d28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:014e3d1a:start=1530922926684824381,finish=1530922926691254464,duration=6430083
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0032d840
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:023af003
$ dmesg | grep -i kill
