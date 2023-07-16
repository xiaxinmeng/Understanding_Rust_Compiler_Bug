plain

[00:04:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:42] tidy error: /checkout/src/test/ui/unsized-enum2.rs:48: line longer than 100 chars
[00:04:44] some tidy checks failed
[00:04:44] 
[00:04:44] 
[00:04:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:44] 
[00:04:44] 
[00:04:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:44] Build completed unsuccessfully in 0:01:51
[00:04:44] Build completed unsuccessfully in 0:01:51
[00:04:44] make: *** [tidy] Error 1
[00:04:44] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16a6e914
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
