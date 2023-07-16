plain

[00:05:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:11] tidy error: /checkout/src/librustdoc/clean/mod.rs:1722: line longer than 100 chars
[00:05:12] some tidy checks failed
[00:05:12] 
[00:05:12] 
[00:05:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:12] 
[00:05:12] 
[00:05:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:12] Build completed unsuccessfully in 0:02:01
[00:05:12] Build completed unsuccessfully in 0:02:01
[00:05:12] Makefile:79: recipe for target 'tidy' failed
[00:05:12] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2aa5815c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
