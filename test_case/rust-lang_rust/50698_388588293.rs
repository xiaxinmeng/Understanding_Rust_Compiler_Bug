plain

[00:05:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:13] tidy error: /checkout/src/librustc_trans/back/link.rs:324: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/test.rs:276: line longer than 100 chars
[00:05:14] Dependencies not on the whitelist:
[00:05:14] * tempfile 
[00:05:14] some tidy checks failed
[00:05:14] 
[00:05:14] 
[00:05:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:14] 
[00:05:14] 
[00:05:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:14] Build completed unsuccessfully in 0:02:07
[00:05:14] Build completed unsuccessfully in 0:02:07
[00:05:14] Makefile:79: recipe for target 'tidy' failed
[00:05:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aa7edd5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
