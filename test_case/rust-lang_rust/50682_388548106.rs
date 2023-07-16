plain

[00:05:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:27] tidy error: /checkout/src/librustc_lint/builtin.rs:1595: line longer than 100 chars
[00:05:28] some tidy checks failed
[00:05:28] 
[00:05:28] 
[00:05:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:28] 
[00:05:28] 
[00:05:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:28] Build completed unsuccessfully in 0:02:15
[00:05:28] Build completed unsuccessfully in 0:02:15
[00:05:28] make: *** [tidy] Error 1
[00:05:28] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:089e33f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
