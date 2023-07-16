plain

[00:05:37] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:38] tidy error: /checkout/src/librustc/traits/select.rs:1179: line longer than 100 chars
[00:05:39] some tidy checks failed
[00:05:39] 
[00:05:39] 
[00:05:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:39] 
[00:05:39] 
[00:05:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:39] Build completed unsuccessfully in 0:02:28
[00:05:39] Build completed unsuccessfully in 0:02:28
[00:05:39] Makefile:79: recipe for target 'tidy' failed
[00:05:39] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b01853c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
