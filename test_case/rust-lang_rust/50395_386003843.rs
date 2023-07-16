plain

[00:05:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:09] tidy error: /checkout/src/librustc_trans/mir/mod.rs:574: trailing whitespace
[00:05:11] some tidy checks failed
[00:05:11] 
[00:05:11] 
[00:05:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:11] 
[00:05:11] 
[00:05:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:11] Build completed unsuccessfully in 0:02:06
[00:05:11] Build completed unsuccessfully in 0:02:06
[00:05:11] make: *** [tidy] Error 1
[00:05:11] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01882036
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
