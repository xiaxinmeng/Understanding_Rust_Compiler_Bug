plain

[00:04:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:51] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:872: trailing whitespace
[00:04:51] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:873: trailing whitespace
[00:04:51] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:874: trailing whitespace
[00:04:52] some tidy checks failed
[00:04:52] 
[00:04:52] 
[00:04:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:52] 
[00:04:52] 
[00:04:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:52] Build completed unsuccessfully in 0:01:59
[00:04:52] Build completed unsuccessfully in 0:01:59
[00:04:52] make: *** [tidy] Error 1
[00:04:52] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e0be218
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
