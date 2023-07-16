plain

[00:05:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:29] tidy error: /checkout/src/test/ui/const-eval/ice-packed.rs: missing trailing newline
[00:05:30] some tidy checks failed
[00:05:30] 
[00:05:30] 
[00:05:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:30] 
[00:05:30] 
[00:05:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:30] Build completed unsuccessfully in 0:02:10
[00:05:30] Build completed unsuccessfully in 0:02:10
[00:05:30] make: *** [tidy] Error 1
[00:05:30] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a816488
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
