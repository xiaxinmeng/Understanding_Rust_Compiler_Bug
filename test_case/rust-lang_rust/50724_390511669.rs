plain

[00:04:55] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:56] tidy error: /checkout/src/librustc/traits/error_reporting.rs:862: line longer than 100 chars
[00:04:57] some tidy checks failed
[00:04:57] 
[00:04:57] 
[00:04:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:57] 
[00:04:57] 
[00:04:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:57] Build completed unsuccessfully in 0:02:02
[00:04:57] Build completed unsuccessfully in 0:02:02
[00:04:57] make: *** [tidy] Error 1
[00:04:57] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:232d1248
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
