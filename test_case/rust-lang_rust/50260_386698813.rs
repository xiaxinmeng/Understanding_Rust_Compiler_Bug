plain

[00:10:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:10:09] tidy error: /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs: incorrect license
[00:10:09] tidy error: /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs: missing trailing newline
[00:10:11] some tidy checks failed
[00:10:11] 
[00:10:11] 
[00:10:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:10:11] 
[00:10:11] 
[00:10:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:10:11] Build completed unsuccessfully in 0:01:56
[00:10:11] Build completed unsuccessfully in 0:01:56
[00:10:11] Makefile:79: recipe for target 'tidy' failed
[00:10:11] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3b0494b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
