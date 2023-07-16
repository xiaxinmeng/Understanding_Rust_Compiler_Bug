plain
[00:03:52]    Compiling tidy v0.1.0 (file:///checkout/src/tools/tidy)
[00:04:00] some tidy checks failed
[00:04:00] 
[00:04:00] 
[00:04:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:00] 
[00:04:00] 
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:00] Build completed unsuccessfully in 0:00:49
[00:04:00] Build completed unsuccessfully in 0:00:49
[00:04:00] Makefile:79: recipe for target 'tidy' failed
[00:04:00] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09a4a148
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
