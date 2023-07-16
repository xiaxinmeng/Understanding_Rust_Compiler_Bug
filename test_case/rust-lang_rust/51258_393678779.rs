plain
tidy check
[00:04:46] * 534 error codes
[00:04:46] * highest error code: E0909
[00:04:46] * 181 features
[00:04:47] Stray file with UI testing output: "/checkout/src/test/ui/borrowck/issue-51117.nll.stderr"
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:40
[00:04:47] Build completed unsuccessfully in 0:01:40
[00:04:47] Makefile:79: recipe for target 'tidy' failed
[00:04:47] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1512a893
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
