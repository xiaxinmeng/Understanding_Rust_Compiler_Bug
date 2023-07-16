
[00:03:27] Expected a gate test for the feature 'on_unimplemented'.
[00:03:27] Hint: create a file named 'feature-gate-on_unimplemented.rs' in the compile-fail
[00:03:27]       test suite, with its failures due to missing usage of
[00:03:27]       #![feature(on_unimplemented)].
[00:03:27] Hint: If you already have such a test and don't want to rename it,
[00:03:27]       you can also add a // gate-test-on_unimplemented line to the test file.
[00:03:27] tidy error: Found 1 features without a gate test.
[00:03:27] some tidy checks failed
[00:03:27] 
[00:03:27] 
[00:03:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/tidy" "/checkout/src" "--no-vendor" "--quiet"
[00:03:27] expected success, got: exit code: 1
[00:03:27] 
[00:03:27] 
[00:03:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:27] Build completed unsuccessfully in 0:01:00
[00:03:27] make: *** [tidy] Error 1
[00:03:27] Makefile:74: recipe for target 'tidy' failed
