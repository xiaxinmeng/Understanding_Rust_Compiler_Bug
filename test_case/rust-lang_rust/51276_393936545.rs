plain

[00:08:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:08:27] tidy error: /checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs: incorrect license
[00:08:28] some tidy checks failed
[00:08:28] 
[00:08:28] 
[00:08:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:08:28] 
[00:08:28] 
[00:08:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:08:28] Build completed unsuccessfully in 0:02:24
[00:08:28] Build completed unsuccessfully in 0:02:24
[00:08:28] Makefile:79: recipe for target 'tidy' failed
[00:08:28] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:036f2bc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
