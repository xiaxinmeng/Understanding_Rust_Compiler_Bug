ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:04:30] 
[00:04:31] some tidy checks failed
[00:04:31] 
[00:04:31] 
[00:04:31] 
[00:04:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:31] 
[00:04:31] 
[00:04:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:31] Build completed unsuccessfully in 0:01:43
[00:04:31] Build completed unsuccessfully in 0:01:43
[00:04:31] make: *** [tidy] Error 1
[00:04:31] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a791114
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
