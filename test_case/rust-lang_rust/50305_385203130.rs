plain

[00:04:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:31] tidy error: /checkout/src/test/rustdoc/mod-stackoverflow.rs: missing trailing newline
[00:04:32] some tidy checks failed
[00:04:32] 
[00:04:32] 
[00:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:32] 
[00:04:32] 
[00:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:32] Build completed unsuccessfully in 0:01:53
[00:04:32] Build completed unsuccessfully in 0:01:53
[00:04:32] make: *** [tidy] Error 1
[00:04:32] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e1b706e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
