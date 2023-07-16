plain
tidy check
[00:04:58] * 543 error codes
[00:04:58] * highest error code: E0911
[00:04:59] * 201 features
[00:04:59] Empty file with UI testing output: "/checkout/src/test/ui/trivial-bounds-inconsistent-associated-functions.stderr"
[00:04:59] some tidy checks failed
[00:04:59] 
[00:04:59] 
[00:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:59] 
[00:04:59] 
[00:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:59] Build completed unsuccessfully in 0:01:56
[00:04:59] Build completed unsuccessfully in 0:01:56
[00:04:59] make: *** [tidy] Error 1
[00:04:59] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06ea8151
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
