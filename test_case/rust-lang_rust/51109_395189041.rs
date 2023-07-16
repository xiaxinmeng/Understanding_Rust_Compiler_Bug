plain

[00:04:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:25] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:1056: line longer than 100 chars
[00:04:26] some tidy checks failed
[00:04:26] 
[00:04:26] 
[00:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:26] 
[00:04:26] 
[00:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:26] Build completed unsuccessfully in 0:01:40
[00:04:26] Build completed unsuccessfully in 0:01:40
[00:04:26] make: *** [tidy] Error 1
[00:04:26] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b2127d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
