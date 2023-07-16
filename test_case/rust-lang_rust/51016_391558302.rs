plain

[00:05:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:49] tidy error: /checkout/src/libcore/ptr.rs:29: line longer than 100 chars
[00:05:51] some tidy checks failed
[00:05:51] 
[00:05:51] 
[00:05:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:51] 
[00:05:51] 
[00:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:51] Build completed unsuccessfully in 0:02:05
[00:05:51] Build completed unsuccessfully in 0:02:05
[00:05:51] make: *** [tidy] Error 1
[00:05:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05a4f4ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
