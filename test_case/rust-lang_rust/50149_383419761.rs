plain

[00:05:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:43] tidy error: /checkout/src/test/ui/lint/must-use-ops.rs: missing trailing newline
[00:05:43] tidy error: /checkout/src/librustc_lint/unused.rs:96: trailing whitespace
[00:05:44] some tidy checks failed
[00:05:44] 
[00:05:44] 
[00:05:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:44] 
[00:05:44] 
[00:05:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:44] Build completed unsuccessfully in 0:02:32
[00:05:44] Build completed unsuccessfully in 0:02:32
[00:05:44] make: *** [tidy] Error 1
[00:05:44] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0021e6f5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
