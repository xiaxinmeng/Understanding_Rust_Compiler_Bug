plain
tidy check
[00:05:12] * 545 error codes
[00:05:12] * highest error code: E0911
[00:05:13] * 204 features
[00:05:14] Stray file with UI testing output: "/checkout/src/test/ui/error-codes/E0394.stderr"
[00:05:14] Stray file with UI testing output: "/checkout/src/test/ui/error-codes/E0494.stderr"
[00:05:14] some tidy checks failed
[00:05:14] 
[00:05:14] 
[00:05:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:14] 
[00:05:14] 
[00:05:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:14] Build completed unsuccessfully in 0:02:05
[00:05:14] Build completed unsuccessfully in 0:02:05
[00:05:14] Makefile:79: recipe for target 'tidy' failed
[00:05:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03051964
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
