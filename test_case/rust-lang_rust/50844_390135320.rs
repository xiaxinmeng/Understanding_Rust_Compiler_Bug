plain

[00:04:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:46] tidy error: /checkout/src/librustc_driver/lib.rs:703: line longer than 100 chars
[00:04:46] tidy error: /checkout/src/librustc_driver/lib.rs:716: line longer than 100 chars
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:59
[00:04:47] Build completed unsuccessfully in 0:01:59
[00:04:47] Makefile:79: recipe for target 'tidy' failed
[00:04:47] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a3b57ea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
