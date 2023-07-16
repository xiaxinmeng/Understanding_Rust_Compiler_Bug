plain

[00:06:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:06] tidy error: /checkout/src/test/run-pass/backtrace-debuginfo.rs:18: line longer than 100 chars
[00:06:08] some tidy checks failed
[00:06:08] 
[00:06:08] 
[00:06:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:08] 
[00:06:08] 
[00:06:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:08] Build completed unsuccessfully in 0:02:42
[00:06:08] Build completed unsuccessfully in 0:02:42
[00:06:08] make: *** [tidy] Error 1
[00:06:08] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ed435d7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
