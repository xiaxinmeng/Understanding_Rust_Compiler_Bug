plain

[00:04:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:57] tidy error: /checkout/src/test/compile-fail/nll/unused-mut-issue-50343.rs:15: line longer than 100 chars
[00:04:59] some tidy checks failed
[00:04:59] 
[00:04:59] 
[00:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:59] 
[00:04:59] 
[00:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:59] Build completed unsuccessfully in 0:01:59
[00:04:59] Build completed unsuccessfully in 0:01:59
[00:04:59] Makefile:79: recipe for target 'tidy' failed
[00:04:59] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02b0f5fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
