plain

[00:04:53] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:53] tidy error: /checkout/src/librustc_trans_utils/symbol_names.rs:209: line longer than 100 chars
[00:04:54] some tidy checks failed
[00:04:54] 
[00:04:54] 
[00:04:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:54] 
[00:04:54] 
[00:04:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:54] Build completed unsuccessfully in 0:01:58
[00:04:54] Build completed unsuccessfully in 0:01:58
[00:04:54] make: *** [tidy] Error 1
[00:04:54] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e052822
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
