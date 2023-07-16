plain

[00:05:27] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:28] tidy error: /checkout/src/test/ui/rfc-1260-main-reexport/with-args-fn.rs: missing trailing newline
[00:05:28] tidy error: duplicate error code: 135
[00:05:28] tidy error: /checkout/src/librustc/diagnostics.rs:506: E0135: r##"
[00:05:28] tidy error: /checkout/src/librustc/diagnostics.rs:2137: //  E0135,
[00:05:28] tidy error: duplicate error code: 133
[00:05:28] tidy error: /checkout/src/librustc_mir/diagnostics.rs:718: E0133: r##"
[00:05:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1547: E0133: r##"
[00:05:28] tidy error: duplicate error code: 134
[00:05:28] tidy error: /checkout/src/librustc/diagnostics.rs:484: E0134: r##"
[00:05:28] tidy error: /checkout/src/librustc/diagnostics.rs:2136: //  E0134,
[00:05:28] Expected a gate test for the feature 'main_reexport'.
[00:05:28] Hint: create a failing test file named 'feature-gate-main_reexport.rs'
[00:05:28]       in the 'ui' test suite, with its failures due to
[00:05:28]       missing usage of #![feature(main_reexport)].
[00:05:28] Hint: If you already have such a test and don't want to rename it,
[00:05:28]       you can also add a // gate-test-main_reexport line to the test file.
[00:05:28] tidy error: Found 1 features without a gate test.
[00:05:29] some tidy checks failed
[00:05:29] 
[00:05:29] 
[00:05:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:29] 
[00:05:29] 
[00:05:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:29] Build completed unsuccessfully in 0:02:02
[00:05:29] Build completed unsuccessfully in 0:02:02
[00:05:29] Makefile:79: recipe for target 'tidy' failed
[00:05:29] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3f941200
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
