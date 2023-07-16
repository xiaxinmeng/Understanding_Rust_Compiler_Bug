plain

[00:05:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:01] tidy error: /checkout/src/test/run-pass/never_transmute_never.rs:19: tab character
[00:05:01] tidy error: /checkout/src/test/run-pass/never_transmute_never.rs:23: tab character
[00:05:02] some tidy checks failed
[00:05:02] 
[00:05:02] 
[00:05:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:02] 
[00:05:02] 
[00:05:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:02] Build completed unsuccessfully in 0:02:01
[00:05:02] Build completed unsuccessfully in 0:02:01
[00:05:02] make: *** [tidy] Error 1
[00:05:02] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:013ede3e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
