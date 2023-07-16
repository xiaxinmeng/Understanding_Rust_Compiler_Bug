plain

[00:05:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:16] tidy error: /checkout/src/librustc/ty/fold.rs:321: line longer than 100 chars
[00:05:16] tidy error: /checkout/src/librustc/ty/flags.rs:25: line longer than 100 chars
[00:05:17] some tidy checks failed
[00:05:17] 
[00:05:17] 
[00:05:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:17] 
[00:05:17] 
[00:05:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:17] Build completed unsuccessfully in 0:01:56
[00:05:17] Build completed unsuccessfully in 0:01:56
[00:05:17] Makefile:79: recipe for target 'tidy' failed
[00:05:17] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f569d90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
