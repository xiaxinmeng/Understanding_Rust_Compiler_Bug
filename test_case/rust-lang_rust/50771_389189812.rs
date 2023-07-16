plain

[00:05:20] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:20] tidy error: /checkout/src/librustdoc/html/layout.rs:45: line longer than 100 chars
[00:05:22] some tidy checks failed
[00:05:22] 
[00:05:22] 
[00:05:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:22] 
[00:05:22] 
[00:05:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:22] Build completed unsuccessfully in 0:02:07
[00:05:22] Build completed unsuccessfully in 0:02:07
[00:05:22] Makefile:79: recipe for target 'tidy' failed
[00:05:22] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1210f789
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
