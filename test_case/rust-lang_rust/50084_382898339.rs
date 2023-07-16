plain

[00:04:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:2591: TODO is deprecated; use FIXME
[00:04:52] tidy error: /checkout/src/tools/compiletest/src/autofix.rs: incorrect license
[00:04:53] some tidy checks failed
[00:04:53] 
[00:04:53] 
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:53] 
[00:04:53] 
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:53] Build completed unsuccessfully in 0:01:55
[00:04:53] Build completed unsuccessfully in 0:01:55
[00:04:53] Makefile:79: recipe for target 'tidy' failed
[00:04:53] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cbb9d72
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
