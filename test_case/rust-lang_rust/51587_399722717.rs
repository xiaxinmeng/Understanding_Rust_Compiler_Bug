plain
travis_time:start:tidy
tidy check
[00:06:05] * 553 error codes
[00:06:05] * highest error code: E0906
[00:06:05] tidy error: Found 1 features without a gate test.
[00:06:05] Expected a gate test for the feature 'macro_at_most_once_rep'.
[00:06:05] Hint: create a failing test file named 'feature-gate-macro_at_most_once_rep.rs'
[00:06:05]       in the 'ui' test suite, with its failures due to
[00:06:05]       missing usage of #![feature(macro_at_most_once_rep)].
[00:06:05] Hint: If you already have such a test and don't want to rename it,
[00:06:05]       you can also add a // gate-test-macro_at_most_once_rep line to the test file.
[00:06:06] some tidy checks failed
[00:06:06] 
[00:06:06] 
[00:06:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:06] 
[00:06:06] 
[00:06:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:06] Build completed unsuccessfully in 0:01:54
[00:06:06] Build completed unsuccessfully in 0:01:54
[00:06:06] Makefile:79: recipe for target 'tidy' failed
[00:06:06] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02a807e4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00f9ac0b:start=1529804763115039036,finish=1529804763120905889,duration=5866853
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b0e3cc9
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09573790
$ dmesg | grep -i kill
