ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:05:14] 
[00:05:14] 
[00:05:14] tidy error: /checkout/src/libcore/task.rs:286: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/test/run-pass/futures-api.rs:19: line longer than 100 chars
[00:05:15] some tidy checks failed
[00:05:15] 
[00:05:15] 
[00:05:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:15] 
[00:05:15] 
[00:05:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:15] Build completed unsuccessfully in 0:02:00
[00:05:15] Build completed unsuccessfully in 0:02:00
[00:05:15] Makefile:79: recipe for target 'tidy' failed
[00:05:15] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f172ae8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
