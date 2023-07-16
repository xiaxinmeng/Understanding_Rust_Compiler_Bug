plain

[00:05:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:22] tidy error: /checkout/src/librustc_lint/unused.rs:499: line longer than 100 chars
[00:05:24] some tidy checks failed
[00:05:24] 
[00:05:24] 
[00:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:24] 
[00:05:24] 
[00:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:24] Build completed unsuccessfully in 0:02:12
[00:05:24] Build completed unsuccessfully in 0:02:12
[00:05:24] make: *** [tidy] Error 1
[00:05:24] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:019ee4d5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
