plain
travis_time:start:tidy
tidy check
[00:05:47] * 538 error codes
[00:05:47] * highest error code: E0911
[00:05:48] Expected a gate test for the feature 'better_divergence_checking'.
[00:05:48] Hint: create a failing test file named 'feature-gate-better_divergence_checking.rs'
[00:05:48]       in the 'ui' test suite, with its failures due to
[00:05:48]       missing usage of #![feature(better_divergence_checking)].
[00:05:48] Hint: If you already have such a test and don't want to rename it,
[00:05:48]       you can also add a // gate-test-better_divergence_checking line to the test file.
[00:05:48] tidy error: Found 1 features without a gate test.
[00:05:49] some tidy checks failed
[00:05:49] 
[00:05:49] 
[00:05:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:49] 
[00:05:49] 
[00:05:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:49] Build completed unsuccessfully in 0:02:31
[00:05:49] Build completed unsuccessfully in 0:02:31
[00:05:49] make: *** [tidy] Error 1
[00:05:49] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f513c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
