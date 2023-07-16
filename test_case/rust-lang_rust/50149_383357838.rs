plain

[00:05:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:51] tidy error: /checkout/src/librustc_lint/unused.rs:95: trailing whitespace
[00:05:51] tidy error: /checkout/src/librustc_lint/unused.rs:98: trailing whitespace
[00:05:51] tidy error: /checkout/src/librustc_lint/unused.rs:99: trailing whitespace
[00:05:51] tidy error: /checkout/src/librustc_lint/unused.rs:115: trailing whitespace
[00:05:53] some tidy checks failed
[00:05:53] 
[00:05:53] 
[00:05:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:53] 
[00:05:53] 
[00:05:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:53] Build completed unsuccessfully in 0:02:33
[00:05:53] Build completed unsuccessfully in 0:02:33
[00:05:53] Makefile:79: recipe for target 'tidy' failed
[00:05:53] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:136cc378
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
