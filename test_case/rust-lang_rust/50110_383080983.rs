plain

[00:04:39] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:40] tidy error: /checkout/src/librustc_lint/builtin.rs:1445: trailing whitespace
[00:04:41] some tidy checks failed
[00:04:41] 
[00:04:41] 
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] 
[00:04:41] 
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:01:46
[00:04:41] Build completed unsuccessfully in 0:01:46
[00:04:41] Makefile:79: recipe for target 'tidy' failed
[00:04:41] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1252e237
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
