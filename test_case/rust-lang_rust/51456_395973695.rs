plain

[00:05:02] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:02] tidy error: /checkout/src/librustc_resolve/lib.rs:4240: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/test/ui/crate-in-paths.rs: incorrect license
[00:05:04] some tidy checks failed
[00:05:04] 
[00:05:04] 
[00:05:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:04] 
[00:05:04] 
[00:05:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:04] Build completed unsuccessfully in 0:01:57
[00:05:04] Build completed unsuccessfully in 0:01:57
[00:05:04] Makefile:79: recipe for target 'tidy' failed
[00:05:04] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:110667db
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
