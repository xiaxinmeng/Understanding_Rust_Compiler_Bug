plain

[00:05:17] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:17] tidy error: /checkout/src/libstd/panic.rs:329: trailing whitespace
[00:05:17] tidy error: /checkout/src/libstd/panic.rs:331: trailing whitespace
[00:05:19] some tidy checks failed
[00:05:19] 
[00:05:19] 
[00:05:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:19] 
[00:05:19] 
[00:05:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:19] Build completed unsuccessfully in 0:01:46
[00:05:19] Build completed unsuccessfully in 0:01:46
[00:05:19] Makefile:79: recipe for target 'tidy' failed
[00:05:19] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0376cc26
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
