plain
tidy check
[00:04:29] * 538 error codes
[00:04:29] * highest error code: E0911
[00:04:30] * 197 features
[00:04:30] Stray file with UI testing output: "/checkout/src/test/ui/issue-43197.stderr"
[00:04:30] Stray file with UI testing output: "/checkout/src/test/ui/conditional_array_execution.stderr"
[00:04:30] some tidy checks failed
[00:04:30] 
[00:04:30] 
[00:04:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:30] 
[00:04:30] 
[00:04:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:30] Build completed unsuccessfully in 0:01:51
[00:04:30] Build completed unsuccessfully in 0:01:51
[00:04:30] Makefile:79: recipe for target 'tidy' failed
[00:04:30] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:106b3d48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
