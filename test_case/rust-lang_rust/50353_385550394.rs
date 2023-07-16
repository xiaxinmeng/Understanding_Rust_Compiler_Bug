plain

[00:06:16] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:16] tidy error: /checkout/src/test/run-pass/nll/issue-50343.rs: missing trailing newline
[00:06:18] some tidy checks failed
[00:06:18] 
[00:06:18] 
[00:06:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:18] 
[00:06:18] 
[00:06:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:18] Build completed unsuccessfully in 0:02:20
[00:06:18] Build completed unsuccessfully in 0:02:20
[00:06:18] Makefile:79: recipe for target 'tidy' failed
[00:06:18] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d46020
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
