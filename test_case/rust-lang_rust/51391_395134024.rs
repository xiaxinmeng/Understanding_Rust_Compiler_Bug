plain

[00:08:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:08:26] tidy error: /checkout/src/librustdoc/clean/mod.rs:1295: line longer than 100 chars
[00:08:27] some tidy checks failed
[00:08:27] 
[00:08:27] 
[00:08:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:08:27] 
[00:08:27] 
[00:08:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:08:27] Build completed unsuccessfully in 0:01:50
[00:08:27] Build completed unsuccessfully in 0:01:50
[00:08:27] make: *** [tidy] Error 1
[00:08:27] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09fa0416
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
