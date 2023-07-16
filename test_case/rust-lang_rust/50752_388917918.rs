plain

[00:06:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:46] tidy error: /checkout/src/test/ui/E0663.rs: missing trailing newline
[00:06:48] some tidy checks failed
[00:06:48] 
[00:06:48] 
[00:06:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:48] 
[00:06:48] 
[00:06:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:48] Build completed unsuccessfully in 0:02:55
[00:06:48] Build completed unsuccessfully in 0:02:55
[00:06:48] make: *** [tidy] Error 1
[00:06:48] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24db3c38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
