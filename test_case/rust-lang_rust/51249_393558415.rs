plain

[00:05:14] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:14] tidy error: /checkout/src/test/ui/suggestions/issue-51244.rs: missing trailing newline
[00:05:16] some tidy checks failed
[00:05:16] 
[00:05:16] 
[00:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:16] 
[00:05:16] 
[00:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:16] Build completed unsuccessfully in 0:01:48
[00:05:16] Build completed unsuccessfully in 0:01:48
[00:05:16] Makefile:79: recipe for target 'tidy' failed
[00:05:16] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cdac5b1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
