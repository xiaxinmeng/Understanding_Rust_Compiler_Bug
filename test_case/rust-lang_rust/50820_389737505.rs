plain
travis_time:start:tidy
tidy check
[00:04:39] * 542 error codes
[00:04:39] * highest error code: E0911
[00:04:40] tidy error: Found 1 features without a gate test.
[00:04:40] Expected a gate test for the feature 'proc_macro_gen'.
[00:04:40] Hint: create a failing test file named 'feature-gate-proc_macro_gen.rs'
[00:04:40]       in the 'ui' test suite, with its failures due to
[00:04:40]       missing usage of #![feature(proc_macro_gen)].
[00:04:40] Hint: If you already have such a test and don't want to rename it,
[00:04:40]       you can also add a // gate-test-proc_macro_gen line to the test file.
[00:04:40] some tidy checks failed
[00:04:40] 
[00:04:40] 
[00:04:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:40] 
[00:04:40] 
[00:04:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:40] Build completed unsuccessfully in 0:01:47
[00:04:40] Build completed unsuccessfully in 0:01:47
[00:04:40] Makefile:79: recipe for target 'tidy' failed
[00:04:40] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04c5039e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
